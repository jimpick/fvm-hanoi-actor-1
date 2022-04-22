fvm-hanoi-actor-1
===

An experiment in writing a Filecoin Virtual Machine smart contract using Rust. Play [Towers of Hanoi](https://en.wikipedia.org/wiki/Tower_of_Hanoi), by installing a custom actor (aka. smart contract) that stores the state in your local devnet Lotus node's blockstore.

For more info on the FVM:

* https://fvm.filecoin.io/

## Video

![Animated GIF](https://ipfs.io/ipfs/QmdW8AXRYAfCNQ3aZHSSc2FWRkEGxJXewqZTrT7Asjcs4m/fvm-hanoi.gif)

[Full Video - 7.4 MB](https://ipfs.io/ipfs/QmdW8AXRYAfCNQ3aZHSSc2FWRkEGxJXewqZTrT7Asjcs4m/fvm-hanoi.mp4)

## "Hello World" Instructions

This is based on @raulk's Hello World example:

Raul's Steps for demo (from Slack Channel)

* Compile actor to Wasm
  * Hello World actor: https://github.com/raulk/fil-hello-world-actor
* Set up a Lotus devnet
  * Experimental branch: experimental/fvm-m2
  * Instructions: https://lotus.filecoin.io/developers/local-network/
  * Pre-built container images for Docker/Kubernetes: https://github.com/jimpick/lotus-fvm-localnet
* Install Actor
  * lotus chain install-actor <path-to-wasm-bytecode>
* Instantiate Actor
  * lotus chain create-actor <code-cid> <encoded-params>
* Invoke actor
  * lotus chain invoke <address> <method_num>

## Setup

Once you have a Lotus devnet setup, make sure it is running and the `lotus` binary is in your path. There is an included `.env` file that you can edit to modify the path for the scripts below.

## Build the WASM bundle for the actor

`cargo build`

(you will need to setup Rust using [rustup](https://rustup.rs/) first.)

## Install the actor bundle into your Lotus devnet

```
$ ./install-actor.sh 
+ tee /var/folders/cd/vcx3gsyd1bs8x0m8jqpnj7s80000gn/T/hanoi.XXXXXXX.mN96EjY8/output.log
+ lotus chain install-actor target/debug/wbuild/hanoi_actor_1/hanoi_actor_1.compact.wasm
sending message...
gas limit: 752107916
waiting for message to execute...
Actor Code CID: bafk2bzaceanrv7ylrgg7vyieamdflg2g33g3oipnqnf4tkdihp26d656ivsy2
Installed: false
Next step:
./create-actor.sh bafk2bzaceanrv7ylrgg7vyieamdflg2g33g3oipnqnf4tkdihp26d656ivsy2
```

## Create an instance of the actor

The "CID" of your actor will be different. Copy the output from the previous command.

```
$ ./create-actor.sh bafk2bzaceanrv7ylrgg7vyieamdflg2g33g3oipnqnf4tkdihp26d656ivsy2
Code CID: bafk2bzaceanrv7ylrgg7vyieamdflg2g33g3oipnqnf4tkdihp26d656ivsy2
Encoded Params:
+ lotus chain create-actor bafk2bzaceanrv7ylrgg7vyieamdflg2g33g3oipnqnf4tkdihp26d656ivsy2
+ tee /var/folders/cd/vcx3gsyd1bs8x0m8jqpnj7s80000gn/T/hanoi.XXXXXXX.wQ1XVwx3/output.log
sending message...
waiting for message to execute...
ID Address: t01028
Robust Address: t2kvncfjy7cmhv7x5lup434gjsopistjbqkbvwimq
Next step:
./invoke.sh t01028 <method num> <encoded-params>


eg. ./invoke.sh t01028 2   # Get state
    ./invoke.sh t01028 3 $(echo 12 | base64)   # Move disc from tower 1 to tower 2
    ./play.sh t01028   # GUI for game written in bash
```

When you create an instance, you actor will have a Filecoin address (eg. "t01028" here)
that you can use to play the game by calling the methods on the smart contract.

You can create as many instances as you want. Each instance is a new game.

## Get the state from your actor (method 2)

The address will be different, so copy the output from the previous command.

```
$ ./invoke.sh t01028 2
Address: t01028
Method: 2
Params:
+ lotus chain invoke t01028 2
+ tee /var/folders/cd/vcx3gsyd1bs8x0m8jqpnj7s80000gn/T/hanoi.XXXXXXX.FbyjdKT0/output.log
sending message...
waiting for message to execute...
eDlTdGF0ZSB7IHRvd2VyMTogWzUsIDQsIDMsIDIsIDFdLCB0b3dlcjI6IFtdLCB0b3dlcjM6IFtdIH0=
Decoded Output: { tower1: [5, 4, 3, 2, 1], tower2: [], tower3: [] }
```

You can see the "decoded output" contains the state of the game, which consists of a number of discs stacked on three towers.

## Move a disc from one tower to another (method 3)

Again, the address will be different, so use your address. The argument in this
case is "12" which means "move a disc from tower 1 to tower 2".

If you want to move the disc from tower 1 to tower 3, use "13". Likewise, to
move a disc from tower 2 to tower 3, use "23".

```
$ ./invoke.sh t01028 3 $(echo 12 | base64)
Address: t01028
Method: 3
Params: MTIK
+ lotus chain invoke t01028 3 MTIK
+ tee /var/folders/cd/vcx3gsyd1bs8x0m8jqpnj7s80000gn/T/hanoi.XXXXXXX.g3sqHSc9/output.log
sending message...
waiting for message to execute...
eDdTdGF0ZSB7IHRvd2VyMTogWzUsIDQsIDMsIDJdLCB0b3dlcjI6IFsxXSwgdG93ZXIzOiBbXSB9
Decoded Output: { tower1: [5, 4, 3, 2], tower2: [1], tower3: [] }
```

After the move is executed, you can see that the state has changed -- disc "1" has moved from tower1 to tower2.

## Play the game interactively in your terminal

There is a bash script that will display the state graphically and allow you to move discs from tower to tower until you win. Use Ctrl-C to quit. Resume anytime.

```
$ ./play.sh t01028
```

...

```
Decoded Output: { tower1: [5, 4, 3, 2], tower2: [1], tower3: [] }

Towers:

       |         |         |    
      —--        |         |    
     --—--       |         |    
    ----—--      |         |    
   ------—--     —         |    
       1         2         3

Move disc from: 1
Move disc to: 3
```

## License

* https://github.com/raulk/fil-hello-world-actor/issues
