# Experiments and Evaluation of ZK Backends

We present our experiments and evaluation of several ZK backends.

## Benchmark Set Description

We use two benchmark sets for our evaluation:
Direct Implementation and Proofs of Proofs.

### Direct Implementation

In this category, we consider four simple programs in the areas of blockchain and AI
and use several ZK backends to directly generate their ZK proofs.
These programs are:
- `transfer`: a simplified version of the ERC20 transfer function;
- `batch-transfer`: a while loop that executes `transfer` for 5000 times;
- `perceptron`: a single-layer [perceptron](https://en.wikipedia.org/wiki/Perceptron).
- `svm`: a [support vector machine (SVM)](https://en.wikipedia.org/wiki/Support_vector_machine)
  model.

The reference pseudocode of these examples are available at the end
of this document.

Given a ZK backend, we directly implement these programs in the
programming language to the backend and generate ZK proof
of their execution traces.

### Proofs of Proofs

In this category, we combine ZK proofs and logical/mathematical proofs.
For a program `Pgm` in a programming language `PL`, we use the
[K framework](https://kframework.org) to generate
a logical proof `PI(Pgm, PL)` that shows the correctness of an execution
trace of `Pgm` using directly a formal semantics of `PL`.
Such a logical proof can be automatically checked by a logical proof checker
`proof_check(PI(Pgm, PL))`.
Then, we generate a ZK proof that shows the correctness of
an execution trace of `proof_check`.
In other words, we generate a ZK proof that shows the correctness
of a logical proof that shows the correctness of `Pgm` written in language `PL`.
Thus, we call this benchmark set Proofs of Proofs, as we generate
(ZK) proofs of (logical) proofs.

### ZK Backends

We consider the following ZK backends:
- [Cairo](https://www.cairo-lang.org/)
- [Lurk](https://lurk-lang.org/)
- [RISC Zero](https://www.risczero.com/)
- [zkLLVM](https://github.com/NilFoundation/zkLLVM)

## Performance Tables

- machine spec: AMD Ryzen 9 7950X(16 cores/32 threads/128GB), 4090RTX
- memory swap: 108GB
- performance time measured in seconds
- besides RISC Zero, all other implementations were executed using the diff between
  the timestamps of the start and end of each possible execution phase.
- RISC Zero has its own performance counter, so we use it to measure the execution time and cycles.
- "CPU" prefix = without GPU acceleration
- "GPU" prefix = with GPU acceleration
- last update date: Dec 19th, 2023.

### Direct Implementation

#### Cairo Zero (v0.13.0)*
Last Update: Dec 22th, 2023
|                                                             Examples                                                             | CPU Exec Time | CPU Prove Time | CPU Verify Time | CPU Total Time |
|:---------------------------------------------------------------------------------------------------------------------------------|:-------------:|:--------------:|:---------------:|:--------------:|
| [transfer](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo0/transfer.cairo)              |         0.440 |          0.195 |           0.008 |          0.643 |
| [batch-transfer](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo0/batch_transfer.cairo)  |         6.825 |         30.196 |           0.869 |         37.890 |
| [perceptron](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo0/perceptron.cairo)          |         0.448 |          0.166 |           0.008 |          0.662 |
| [svm](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo0/svm.cairo)                        |         0.443 |          0.176 |           0.008 |          0.627 |

\* The programs were compiled with Cairo Zero's default compiler and proved and verified using  Lambdaworks Cairo Platinum Prover (v0.3.0)


#### Cairo One (v2.3.1)*
Last Update: Jan 10th, 2024
|                                                             Examples                                                             | CPU Exec Time | CPU Prove Time | CPU Verify Time | CPU Total Time |
|:---------------------------------------------------------------------------------------------------------------------------------|:-------------:|:--------------:|:---------------:|:--------------:|
| [transfer](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo1/transfer.cairo)              |         0.583 |          0.009 |           0.002 |          0.594 |
| [batch-transfer](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo1/batch_transfer.cairo)  |         0.691 |         65.693 |           1.787 |         68.171 |
| [perceptron](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo1/perceptron.cairo)          |         0.592 |          0.029 |           0.003 |          0.624 |
| [svm](https://github.com/runtimeverification/proof-checker/blob/main/cairo/tests/direct-implementation/cairo1/svm.cairo)                        |         0.593 |          0.032 |           0.003 |          0.628 |

\* The programs were compiled with LambdaClass Cairo-VM (v1.0.0-rc0) and proved and verified using Lambdaworks Cairo Platinum Prover (v0.3.0)


#### Lurk (v0.3.1)
Last Update: Dec 19th, 2023
|                                                       Examples                                                          | Iterations | CPU Prove Time | CPU Verify Time | CPU Total Time | GPU Prove Time | GPU Verify Time | GPU Total Time |
|:-----------------------------------------------------------------------------------------------------------------------:|:----------:|:--------------:|:---------------:|:--------------:|:--------------:|:---------------:|:--------------:|
| [transfer](https://github.com/runtimeverification/proof-checker/blob/main/lurk/tests/direct-implementation/transfer.lurk)              |         34 |          2.393 |           0.554 |          2.947 |          2.313 |           0.618 |          2.931 |
| [batch-transfer](https://github.com/runtimeverification/proof-checker/blob/main/lurk/tests/direct-implementation/batch_transfer.lurk)* |     505037 |       3681.819 |           9.845 |       3691.664 |       1193.355 |           6.571 |       1199.926 |
| [perceptron](https://github.com/runtimeverification/proof-checker/blob/main/lurk/tests/direct-implementation/perceptron.lurk)          |         11 |          3.501 |           0.541 |          4.042 |          0.830 |           0.579 |          1.409 |
| [svm](https://github.com/runtimeverification/proof-checker/blob/main/lurk/tests/direct-implementation/svm5.lurk)                       |          9 |          1.832 |           0.538 |          2.370 |          0.820 |           0.598 |          1.418 |

\* Using `lurk --rc 400 batch_transfer.lurk`, other tests doesn't use `--rc`


#### RISC Zero (v0.16.1)
Last Update: Dec 22th, 2023
|                                                         Examples                                                              |  Cycles | CPU Exec Time | GPU Exec Time | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:-----------------------------------------------------------------------------------------------------------------------------:|:-------:|:-------------:|:-------------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| [transfer](https://github.com/runtimeverification/proof-checker/blob/main/risc0/tests/direct-implementation/guest/src/transfer.rs)           |  21156  |     0.017     |     0.030     |      2.353     |      0.613     |      0.001      |      0.002      |      2.371     |      0.645     |
| [batch-transfer](https://github.com/runtimeverification/proof-checker/blob/main/risc0/tests/direct-implementation/guest/src/transfer5000.rs) | 754199  |     0.057     |     0.057     |     37.878     |      7.532     |      0.002      |      0.001      |     37.937     |      7.590     |
| [perceptron](https://github.com/runtimeverification/proof-checker/blob/main/risc0/tests/direct-implementation/guest/src/perceptron.rs)       |  21156  |     0.017     |     0.028     |      2.355     |      0.595     |      0.001      |      0.002      |      2.373     |      0.625     |
| [svm](https://github.com/runtimeverification/proof-checker/blob/main/risc0/tests/direct-implementation/guest/src/svm5.rs)                    |  21156  |     0.028     |     0.028     |      2.351     |      0.602     |      0.002      |      0.002      |      2.381     |      0.632     |


#### zkLLVM (v0.1.11-48)
Last Update: Jan 8th, 2024
|                                                  Examples                                                         | CPU Circuit Gen Time | CPU Prove+Verify Time |
|:-----------------------------------------------------------------------------------------------------------------:|:--------------------:|:---------------------:|
| [transfer](https://github.com/runtimeverification/proof-checker/tree/main/zkllvm/tests/direct-implementation/transfer)             |                0.730 |                 0.131 |
| [batch-transfer](https://github.com/runtimeverification/proof-checker/tree/main/zkllvm/tests/direct-implementation/batch_transfer) |                1.367 |               143.183 |
| [perceptron](https://github.com/runtimeverification/proof-checker/tree/main/zkllvm/tests/direct-implementation/perceptron)         |                0.750 |                 0.130 |
| [svm](https://github.com/runtimeverification/proof-checker/tree/main/zkllvm/tests/direct-implementation/svm)                       |                0.730 |                 0.132 |

### Proofs of Proofs

#### Lurk (v0.3.1)
Last Update: Dec 19th, 2023
|                                                            Examples                                                      | Cycles | GPU Prove Time | GPU Verify Time | GPU Total Time | GPU Prove Time | GPU Verify Time | GPU Total Time |
|:------------------------------------------------------------------------------------------------------------------------:|:------:|:--------------:|:---------------:|:--------------:|:--------------:|:---------------:|:--------------:|
| [impreflex](https://github.com/runtimeverification/proof-checker/blob/main/lurk/test_impreflex.lurk)*                    |   55651|        217.268 |           5.800 |        223.068 |        107.558 |           3.967 |        111.525 |
| [transfer-goal](https://github.com/runtimeverification/proof-checker/blob/main/lurk/test_transfer_goal.lurk)             | 3202986|             ∞  |              ∞  |             ∞  |              ∞ |               ∞ |              ∞ |
| [batch-transfer-goal](https://github.com/runtimeverification/proof-checker/blob/main/lurk/test_batch_transfer_goal.lurk) |30122047|             ∞  |              ∞  |             ∞  |              ∞ |               ∞ |              ∞ |
| [perceptron-goal](https://github.com/runtimeverification/proof-checker/blob/main/lurk/test_perceptron_goal.lurk)         | 6404208|             ∞  |              ∞  |             ∞  |              ∞ |               ∞ |              ∞ |
| [svm-goal](https://github.com/runtimeverification/proof-checker/blob/main/lurk/test_svm_goal.lurk)                       | 6404208|             ∞  |              ∞  |             ∞  |              ∞ |               ∞ |              ∞ |


\* Using `lurk --rc 400 ...`


#### RISC Zero (v0.16.1)
Last Update: Dec 22th, 2023
|      Examples*      |  Cycles | CPU Exec Time | GPU Exec Time | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:-------------------:|:-------:|:-------------:|:-------------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| impreflex           |   66366 |     0.031     |     0.030     |       4.754    |      1.256     |      0.001      |      0.001      |       4.786    |      1.287     |
| transfer-goal       | 1139247 |     0.034     |     0.034     |      48.938    |     10.663     |      0.003      |      0.003      |      48.975    |     10.700     |
| batch-transfer-goal | 6724805 |     0.114     |     0.114     |     274.237    |     59.819     |      0.011      |      0.011      |     274.362    |     59.944     |
| perceptron-goal     | 3212346 |     0.049     |     0.050     |     127.911    |     28.433     |      0.006      |      0.006      |     127.966    |     28.489     |
| svm-goal            | 3212346 |     0.069     |     0.050     |     128.289    |     28.695     |      0.006      |      0.006      |     128.364    |     28.751     |

\* For the RISC Zero $PI^2$ implementation, we have the main implementation defined
[here](https://github.com/runtimeverification/proof-checker/tree/main/risc0/pi2)
and the inputs defined [here](https://github.com/runtimeverification/proof-checker/tree/main/proofs/translated).
The inputs are split into three files: `*-gamma`, `*-claim`, and `*-proof`.
Ultimately, we expect that all $PI^2$ implementations will support an unique
binary input format, and therefore, all implementations will share these same
inputs and have only one main implementation.

#### zkLLVM (v0.1.11-48)
Last Update: Jan 8th, 2024
|       Examples      |CPU Circuit Gen Time | CPU Prove+Verify Time |
|:-------------------:|:-------------------:|:---------------------:|
| impreflex           |               9.137 |              1373.576 |
| transfer-goal       |             147.247 |                     ∞ |
| batch-transfer-goal |            1407.846 |                     ∞ |
| perceptron-goal     |             575.171 |                     ∞ |
| svm-goal            |             585.308 |                     ∞ |

\* For the zkLLVM $PI^2$ implementation, we have the main implementation defined
[here](https://github.com/runtimeverification/proof-checker/tree/main/zkllvm/src)
and the inputs defined [here](https://github.com/runtimeverification/proof-checker/tree/main/zkllvm/tests/proofs-of-proofs).
The inputs are split and encoded into three arrays on a file for each file to
match the input requirements of the zkLLVM implementation.

## Implementation Details

### Lurk Implementation Details
Lurk is a interpreted programming language, that said, when we execute an
example in Lurk, we are actually executing the interpreter that will execute the
program. This means that the execution time required for the interpreter to load
(interpret) every function and definition is also counted in the execution time
of the program, and therefore, we can't measure the compilation time of the
program itself.

To execute large Lurk examples requires an increased swap memory, resulting in
slower execution times compared to other implementations. Due to this limitation,
it is difficult to accurately measure and compare execution times between Lurk
and other implementations. Even though we have 128GB of RAM + 108Gb of swap
memory, we still couldn't execute most of $PI^2$ examples in Lurk, that what
the `∞` means on the performance tables.

The `--rc n` flag is used to improve the performance execution of larger
programs in Lurk. The `rc` value is the number of iterations that Lurk packs
together in a single [Nova](https://github.com/microsoft/Nova) folding step.
Iterations in Lurk represents reduction steps in the [Lurk Universal Circuit](https://blog.lurk-lang.org/posts/circuit-spec/).
In terms of parallelism, Lurk is capable of generating more partial witnesses in
parallel with higher rc values. However, the higher the rc value, the more
memory is required to execute the program. The default value of `rc` is 10, and
we used `rc=400` for the `batch-transfer` example. In small cases, a higher `rc`
value can decrease the execution time, that is why we use it for programs with
more than 100K iterations.

The Lurk's examples were executed within the following version:

```bash
commit: 2023-12-21 510d7042990844760d97d65c7e6c7ab75f934630
lurk 0.3.1
```

To execute the examples using GPU this setup was used to compile the Lurk binary:

```bash
export EC_GPU_CUDA_NVCC_ARGS='--fatbin --gpu-architecture=sm_89 --generate-code=arch=compute_89,code=sm_89'
export CUDA_ARCH=89
export NVIDIA_VISIBLE_DEVICES=all
export NVIDIA_DRIVER_CAPABILITITES=compute,utility
export EC_GPU_FRAMEWORK=cuda
cargo install --path . --features=cuda --force
```

To execute the examples using only CPU this setup was used to compile the Lurk
binary:

```bash
export CUDA_PATH=
export NVCC=off
export EC_GPU_FRAMEWORK=none
cargo install --path . --force
```

### RISC Zero Implementation Details

From [RiscZero Terminogy](https://dev.risczero.com/terminology#clock-cycles) the `Cycles` we use in the performance tables are the
smallest unit of compute in the zkVM circuit, analogous to a clock cycle on a
physical CPU. The complexity of a guest program's execution is measured in clock
cycles as they directly affect the memory, proof size, and time performance of
the zkVM.

Generally, a single cycle corresponds to a single RISC-V operation. However,
some operations require two cycles.

### zkLLVM Implementation Details

zkLLVM doesn't support GPU acceleration in any phase, therefore, we don't have
GPU results for these experiments.

The proof and verification on zkLLVM were genereted using
`transpiler -m gen-test-proof`.

The version of the individual tools used to execute the examples were:

```bash
$ clang-17 --version
clang version 17.0.4 (http://www.github.com/NilFoundation/zkllvm-circifier.git 4c393658e71bed430b996cff8555a548fbe8bbda)

$ assigner --version
0.1.11-48

$ transpiler --version
0.1.11-48
```

The `∞` on the performance tables means that the example didn't finish executing
after 6 hours or was killed by the OS due to lack of memory.
