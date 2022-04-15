fvm-hanoi-actor-1
===

Experimenting with Filecoin smart contracts on the FVM!

* https://fvm.filecoin.io/

## "Hello World" Instructions

This is based on @raulk's Hello World example:

Raul's Steps for demo (from Slack Channel)

* Compile actor to Wasm
  * Hello World actor: https://github.com/raulk/fil-hello-world-actor
* Set up a Lotus devnet
  * Experimental branch: experimental/fvm-m2
  * Instructions: https://lotus.filecoin.io/developers/local-network/
* Install Actor
  * lotus chain install-actor <path-to-wasm-bytecode>
* Instantiate Actor
  * lotus chain create-actor <code-cid> <encoded-params>
* Invoke actor
  * lotus chain invoke <address> <method_num>

## License

* https://github.com/raulk/fil-hello-world-actor/issues
