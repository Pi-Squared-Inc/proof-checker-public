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

