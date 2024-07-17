# A Cairo Implementation of the Proof Checker

This directory includes an implementation of the Matching Logic proof checker in the
[Cairo Language](https://www.cairo-lang.org/) (the new Cairo, previously known as
Cairo 1).

## Installing Cairo

Cairo is needed for running and testing the checker. Follow the instructions given in the
["Installation"](https://book.cairo-lang.org/ch01-01-installation.html) section of
the Cairo Book to install Scarb, Cairo's package manager that bundles with it various
Cairo tools, including the Cairo compiler.

Once installed, check your installation by invoking Scarb:
```bash
scarb --version
```

You should get output similar to the following:
```bash
scarb 2.3.1 (0c8def3aa 2023-10-31)
cairo: 2.3.1 (https://crates.io/crates/cairo-lang-compiler/2.3.1)
sierra: 1.3.0
```

We're currently using the Nightly version of Scarb for development.
The current version of the project is in:
```
scarb 2.4.0+nightly-2023-12-14 (be8d24445 2023-12-14)
cairo: 2.4.0 (764e082bc)
sierra: 1.4.0
```

To install it with `asfg`, run:
```bash
asdf install scarb nightly-2023-12-14
asdf global scarb nightly-2023-12-14
```

## Running and Testing the Proof Checker

Make sure you are in the directory of this project `cairo` when running the following commands.

* Build the checker
    ```bash
    scarb build
    ```

* Build and run the checker
    ```bash
    scarb cairo-run
    ```

* Run all tests
    ```bash
    scarb cairo-test
    ```

* Fix formatting automatically:
    ```bash
    scarb fmt
    ```

## Executing, Proving and Verifying CSL-Examples direct implementations
Instead of using `scarb` we'll use `cairo1-run` binary from LambdaClass to
execute the CSL-Examples in Proof Mode to generate the files necessary to
prove and verify the examples.

To get the cairo1-run binary, you need to:
1. Clone `cairo-vm` in a seperate directory

    1.1. `git clone --recursive https://github.com/lambdaclass/cairo-vm`
    
    1.2.  Checkout a version greater than `v0.9.1`

2. Go to `cairo-vm/cairo1-run` directory and run:
```bash
  make deps
  cargo build --release
  cargo install --locked --path .
```
3. Now you can use `cairo1-run` binary to execute cairo programs if Cairo's
   corelib is in the same directory where you're calling the `cairo1-run`
   binary.
4. To Prove and Verify, you need to install `platinum-prover` binary on a
   version equal or greater than `v0.9.1`. You can find the instructions
   [here](https://github.com/lambdaclass/lambdaworks/tree/main/provers/cairo). 
   
