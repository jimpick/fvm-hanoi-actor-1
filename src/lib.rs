mod blockstore;

use crate::blockstore::Blockstore;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::message::NO_DATA_BLOCK_ID;
use fvm_shared::ActorID;

/// A macro to abort concisely.
/// This should be part of the SDK as it's very handy.
macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct State {
    tower1: Vec<u8>,
    tower2: Vec<u8>,
    tower3: Vec<u8>,
}

impl State {
    fn new(n: u8) -> State {
        let mut tower1: Vec<u8> = Vec::new();
        let tower2: Vec<u8> = Vec::new();
        let tower3: Vec<u8> = Vec::new();

        for i in 1..=n {
            tower1.push(i);
        }
        tower1.reverse();

        State { tower1, tower2, tower3 }
    }

    fn is_legal(&self, from: u8, to: u8) -> bool {
        let source = match from {
            1 => &self.tower1,
            2 => &self.tower2,
            3 => &self.tower3,
            _ => { abort!(USR_ILLEGAL_ARGUMENT, "Invalid from value: {}", from); }
        };
        if source.len() == 0 { return false };
        let top_disc = source[source.len() - 1];
        let target = match to {
            1 => &self.tower1,
            2 => &self.tower2,
            3 => &self.tower3,
            _ => { abort!(USR_ILLEGAL_ARGUMENT, "Invalid to value: {}", to); }
        };
        if target.len() == 0 { return true };
        let bottom_disc = target[target.len() - 1];
        if top_disc < bottom_disc { return true }
        else { return false }
    }

    fn move_disc(&mut self, from: u8, to: u8) -> &mut State {
        if !self.is_legal(from, to) {
            abort!(USR_FORBIDDEN, "Illegal move, {} => {}", from, to)
        }
        let source = match from {
            1 => &mut self.tower1,
            2 => &mut self.tower2,
            3 => &mut self.tower3,
            _ => { abort!(USR_ILLEGAL_ARGUMENT, "Invalid from value: {}", from); }
        };
        let disc = source.pop().unwrap();
        let target = match to {
            1 => &mut self.tower1,
            2 => &mut self.tower2,
            3 => &mut self.tower3,
            _ => { abort!(USR_ILLEGAL_ARGUMENT, "Invalid to value: {}", to); }
        };
        target.push(disc);
        self
    }
}

/// We should probably have a derive macro to mark an object as a state object,
/// and have load and save methods automatically generated for them as part of a
/// StateObject trait (i.e. impl StateObject for State).
impl State {
    pub fn load() -> Self {
        // First, load the current state root.
        let root = match sdk::sself::root() {
            Ok(root) => root,
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
        };

        // Load the actor state from the state tree.
        match Blockstore.get_cbor::<Self>(&root) {
            Ok(Some(state)) => state,
            Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
            Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
        }
    }

    pub fn save(&self) -> Cid {
        let serialized = match to_vec(self) {
            Ok(s) => s,
            Err(err) => abort!(USR_SERIALIZATION, "failed to serialize state: {:?}", err),
        };
        let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice())
        {
            Ok(cid) => cid,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
        };
        if let Err(err) = sdk::sself::set_root(&cid) {
            abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
        }
        cid
    }
}

/// The actor's WASM entrypoint. It takes the ID of the parameters block,
/// and returns the ID of the return value block, or NO_DATA_BLOCK_ID if no
/// return value.
///
/// Should probably have macros similar to the ones on fvm.filecoin.io snippets.
/// Put all methods inside an impl struct and annotate it with a derive macro
/// that handles state serde and dispatch.
#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    // Conduct method dispatch. Handle input parameters and return data.
    let ret: Option<RawBytes> = match sdk::message::method_number() {
        1 => constructor(),
        2 => get(),
        3 => {
            let params = sdk::message::params_raw(params).unwrap().1;
            let params = RawBytes::new(params);
            move_disc(params)
        },
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    // Insert the return data block if necessary, and return the correct
    // block ID.
    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

/// The constructor populates the initial state.
///
/// Method num 1. This is part of the Filecoin calling convention.
/// InitActor#Exec will call the constructor on method_num = 1.
pub fn constructor() -> Option<RawBytes> {
    // This constant should be part of the SDK.
    const INIT_ACTOR_ADDR: ActorID = 1;

    // Should add SDK sugar to perform ACL checks more succinctly.
    // i.e. the equivalent of the validate_* builtin-actors runtime methods.
    // https://github.com/filecoin-project/builtin-actors/blob/master/actors/runtime/src/runtime/fvm.rs#L110-L146
    if sdk::message::caller() != INIT_ACTOR_ADDR {
        abort!(USR_FORBIDDEN, "constructor invoked by non-init actor");
    }

    let state = State::new(5);
    state.save();
    None
}

/// Method num 2.
pub fn get() -> Option<RawBytes> {
    let state = State::load();

    let ret = to_vec(format!("{:?}", &state).as_str());
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}

/// Method num 3.
pub fn move_disc(params: RawBytes) -> Option<RawBytes> {
    let mut state = State::load();
    let from = params[0] - b'0';
    let to = params[1] - b'0';
    state.move_disc(from, to);
    state.save();

    let ret = to_vec(format!("{:?}", &state).as_str());
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
