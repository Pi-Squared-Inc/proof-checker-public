# $\Pi^2$ Proof Checkers

This repository contains implementations of the matchign logic proof checker using various ZK implementations.
Note that the code in this repository is experimental and exploratory.

## Risc0

* Run without generating a certificate: `cargo run --bin checker ./proofs/propositional.ml-gamma ./proofs/propositional.ml-claim ./proofs/propositional.ml-proof`
* Generate a cerificate: `cargo run --bin host ./proofs/propositional.ml-gamma ./proofs/propositional.ml-claim ./proofs/propositional.ml-proof`

## zkLLVM

* Build:
* Run without generating a certificate:
* Generate a certificate:

## Cairo

In the `cairo` directory:
* Build the checker: `scarb build`
* Build and run the checker: `scarb cairo-run`
* Run the tests: `scarb cairo-test`

