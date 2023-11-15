$\Pi^2$ Proof Checker Implementation
====================================

This repository contains implementations of the matching logic proof checker
using Cairo, Risc0 and zkLLVM. Note that the code in this repository is
experimental and exploratory.

Cairo
-----

In the `cairo` directory:

-   Build the checker: `scarb build`
-   Build and run the checker: `scarb cairo-run`
-   Run the tests: `scarb cairo-test`

Risc0
-----

-   Run without generating a certificate:
    `cargo run --bin checker ./proofs/propositional.ml-gamma ./proofs/propositional.ml-claim ./proofs/propositional.ml-proof`
-   Generate a cerificate:
    `cargo run --bin host ./proofs/propositional.ml-gamma ./proofs/propositional.ml-claim ./proofs/propositional.ml-proof`

zkLLVM
------

-   Translate binary input to zkllvm input:
    `python3 translator.py <path-to-assumption> <path-to-claim> <path-to-proof>`
-   We have a build script to generate circuit and proof:
    `build.sh src/main.cpp inputs/transfer-simple-compressed-goal.inp`

The build script does the following:

-   Use `clang` to compile the proof checker
-   Use `assigner` to circuit and assignment table.Check the circuit
    satisfiability with the generated assignment table.
-   Use `transpiler` to generate the proof and verify it.

