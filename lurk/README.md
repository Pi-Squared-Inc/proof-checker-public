# Lurk implementation for Proof-Checker

## Setting up Lurk

Lurk has its dependency on the [clang](https://clang.llvm.org/get_started.html).
Please make sure that clang is installed before you install the Lurk.


[lurk-rs](https://github.com/lurk-lab/lurk-rs) is the Rust implementation of Lurk, which generates binaries via rustc.
We can clone the repo and init its submodule by:
```bash
https://github.com/lurk-lab/lurk-rs.git
git submodule update --init --recursive
```

To execute Lurk programs using GPU, this setup was used to compile the Lurk binary:
```bash
export EC_GPU_CUDA_NVCC_ARGS='--fatbin --gpu-architecture=sm_89 --generate-code=arch=compute_89,code=sm_89'
export CUDA_ARCH=89
export NVIDIA_VISIBLE_DEVICES=all
export NVIDIA_DRIVER_CAPABILITITES=compute,utility
export EC_GPU_FRAMEWORK=cuda
cargo install --path . --features=cuda --force
```

To execute Lurk programs using only CPU, this setup was used to compile the Lurk binary:
```bash
export CUDA_PATH=
export NVCC=off
export EC_GPU_FRAMEWORK=none
cargo install --path . --force
```

## Run Lurk program

You can run lurk programs with:
```bash
lurk <lurk_program>
```
You can configure the number of iterations packed in a batch by setting the `--rc` flag.

For example,
```bash
lurk --rc 400 test_impreflex.lurk
```
