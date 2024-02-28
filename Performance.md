# Experiments and Evaluation of ZK Backends

This document outlines our experiments and evaluation of various Zero Knowledge (ZK) backends. Our evaluation uses two benchmark sets: Direct Implementation and Proofs of Proofs.

### Direct Implementation

In this category, we consider four simple programs relevant to blockchain and AI. We use various ZK backends to directly generate their ZK proofs.
The programs include:
- `transfer`: a simplified version of the ERC20 transfer function;
- `batch-transfer`: a while loop that executes `transfer` for 5000 times;
- `perceptron`: a single-layer [perceptron](https://en.wikipedia.org/wiki/Perceptron).
- `svm`: a [support vector machine (SVM)](https://en.wikipedia.org/wiki/Support_vector_machine)
  model.

For each ZK backend, we directly implement these programs in the respective programming language and generate ZK proofs of their execution traces.

### Proofs of Proofs

This category combines ZK proofs and logical/mathematical proofs.
For a program `Pgm` in a programming language `PL`, we use the [K framework](https://kframework.org) to generate a logical proof `PI(Pgm, PL)`. This proof demonstrates the correctness of an execution trace of `Pgm` using a formal semantics of `PL`.

A logical proof checker can automatically verify `proof_check(PI(Pgm, PL))`. We then generate a ZK proof demonstrating the correctness of an execution trace of `proof_check`.

In essence, we generate a ZK proof that validates the correctness of a logical proof, which in turn verifies the correctness of `Pgm` written in the language `PL`. This is why we refer to this benchmark set as Proofs of Proofs--it generates (ZK) proofs of (logical) proofs.

## Performance Tables

Some key details about the performance data for the ZK backends. The evaluations were carried out on an AMD Ryzen 9 7950X machine, with 16 cores, 32 threads, and 128 GB of memory, paired with a 4090RTX GPU and 108 GB of memory swap.

The performance time is measured in seconds, and it's important to note how these times were calculated. For all implementations, except RISC Zero, the execution time was determined by measuring the time difference between the start and end timestamps of each execution phase. RISC Zero, has its own performance counter that was utilized to measure both the execution time and cycles.

We've also noted where the implementation is using CPU or GPU acceleration. If you see the prefix "CPU", this refers to an implementation without GPU acceleration, whereas "GPU" denotes an implementation that utilized GPU acceleration.

--- 
## ZK Backends

### [Lurk](https://lurk-lang.org/) Backend
#### Lurk Implementation Details

Lurk is an interpreted programming language. When we run a Lurk example, we are actually executing the interpreter, which in turn executes the program. The execution time includes the time needed for the interpreter to load every function and definition. Consequently, we cannot measure the compilation time of the program itself.

Executing large Lurk examples requires an increase in swap memory, which results in slower execution times compared to other implementations. This limitation makes it challenging to measure and compare execution times between Lurk and other implementations accurately. Despite having 128GB of RAM plus 108GB of swap memory, we were still unable to execute most of the $PI^2$ examples in Lurk. The symbol `∞` in the performance tables indicates this inability.

The `--rc n` flag in Lurk is used to enhance execution performance of larger programs. The `rc` value indicates the number of iterations that Lurk bundles together in a single [Nova](https://github.com/microsoft/Nova) folding step. Here, iterations represent reduction steps in the [Lurk Universal Circuit](https://blog.lurk-lang.org/posts/circuit-spec/).

In terms of parallelism, higher `rc` values allow Lurk to generate more partial witnesses simultaneously. However, a larger `rc` value also requires more memory to execute the program. The default `rc` value is 10. For the `batch-transfer` example, we used `rc=400`. In smaller cases, increasing the `rc` value can reduce execution time, which is why it's used for programs with
over 100K iterations.

The Lurk examples were run on the following version:

```bash
commit: 2023-12-21 510d7042990844760d97d65c7e6c7ab75f934630
lurk 0.3.1
```

The following setup was used to compile the Lurk binary and run the example using GPU: 

```bash
export EC_GPU_CUDA_NVCC_ARGS='--fatbin --gpu-architecture=sm_89 --generate-code=arch=compute_89,code=sm_89'
export CUDA_ARCH=89
export NVIDIA_VISIBLE_DEVICES=all
export NVIDIA_DRIVER_CAPABILITITES=compute,utility
export EC_GPU_FRAMEWORK=cuda
cargo install --path . --features=cuda --force
```

The following setup was used to compile the Lurk binary and run the example using CPU:

```bash
export CUDA_PATH=
export NVCC=off
export EC_GPU_FRAMEWORK=none
cargo install --path . --force
```

#### Lurk Direct Implementation
<details open>
  <summary>Lurk (v0.3.1)</summary>

|                                                       Examples                                                                         | Iterations | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:--------------------------------------------------------------------------------------------------------------------------------------:|:----------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| [transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/direct-implementation/transfer.lurk)              |         34 |          2.393 |          2.313 |           0.554 |           0.618 |          2.947 |          2.931 |
| [batch-transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/direct-implementation/batch_transfer.lurk)<sup>*</sup> |     505037 |       3681.819 |       1193.355 |           9.845 |           6.571 |       3691.664 |       1199.926 |
| [perceptron](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/direct-implementation/perceptron.lurk)          |         11 |          3.501 |          0.830 |           0.541 |           0.579 |          4.042 |          1.409 |
| [svm](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/direct-implementation/svm.lurk)                        |          9 |          1.832 |          0.820 |           0.538 |           0.598 |          2.370 |          1.418 |


<sup>*</sup> The batch-transfer example utilizes `lurk --rc 400 batch_transfer.lurk`, while other tests do not use the `--rc` flag
</details>

#### Lurk Proofs of Proofs

<details open>
  <summary>Lurk (v0.3.1)</summary>

  |                                                            Examples                                                                             | Cycles | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:-----------------------------------------------------------------------------------------------------------------------------------------------:|:------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| [impreflex](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/proofs-of-proofs/test_impreflex.lurk)<sup>*</sup>                    |   55651|        217.268 |        107.558 |           5.800 |           3.967 |        223.068 |        111.525 |
| [transfer-goal](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/proofs-of-proofs/test_transfer_goal.lurk)             | 3202986|             ∞  |             ∞  |              ∞  |               ∞ |              ∞ |              ∞ |
| [batch-transfer-goal](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/proofs-of-proofs/test_batch_transfer_goal.lurk) |30122047|             ∞  |             ∞  |              ∞  |               ∞ |              ∞ |              ∞ |
| [perceptron-goal](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/proofs-of-proofs/test_perceptron_goal.lurk)         | 6404208|             ∞  |             ∞  |              ∞  |               ∞ |              ∞ |              ∞ |
| [svm-goal](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/lurk/tests/proofs-of-proofs/test_svm_goal.lurk)                       | 6404208|             ∞  |             ∞  |              ∞  |               ∞ |              ∞ |              ∞ |

<sup>*</sup> The impreflex example utilizes `lurk --rc 400 batch_transfer.lurk`, while other tests do not use the `--rc` flag
</details>

---

### [RISC Zero](https://www.risczero.com/) Backend
#### RISC Zero Implementation Details

From the [RiscZero Terminogy](https://dev.risczero.com/terminology#clock-cycles), the `Cycles` we refer to in the performance tables are the smallest unit of computation in the zkVM circuit, similar to a clock cycle on a physical CPU. The execution complexity of a guest program is measured in these clock cycles as they directly impact the memory, proof size, and time performance of
the zkVM.

Generally, a single cycle corresponds to one RISC-V operation. However, some operations may require two cycles.#### RISC Zero Implementation Details

From the [RiscZero Terminogy](https://dev.risczero.com/terminology#clock-cycles), the `Cycles` we refer to in the performance tables are the smallest unit of computation in the zkVM circuit, similar to a clock cycle on a physical CPU. The execution complexity of a guest program is measured in these clock cycles as they directly impact the memory, proof size, and time performance of
the zkVM.

Generally, a single cycle corresponds to one RISC-V operation. However, some operations may require two cycles.

#### RISC Zero Direct Implementation
<details open>
  <summary>RISC Zero (v0.16.1)</summary>
  
|                                                         Examples                                                                             |  Cycles | CPU Exec Time | GPU Exec Time | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:--------------------------------------------------------------------------------------------------------------------------------------------:|:-------:|:-------------:|:-------------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| [transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/risc0/tests/direct-implementation/guest/src/transfer.rs)           |  21156  |     0.017     |     0.030     |      2.353     |      0.613     |      0.001      |      0.002      |      2.371     |      0.645     |
| [batch-transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/risc0/tests/direct-implementation/guest/src/transfer5000.rs) | 754199  |     0.057     |     0.057     |     37.878     |      7.532     |      0.002      |      0.001      |     37.937     |      7.590     |
| [perceptron](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/risc0/tests/direct-implementation/guest/src/perceptron.rs)       |  21156  |     0.017     |     0.028     |      2.355     |      0.595     |      0.001      |      0.002      |      2.373     |      0.625     |
| [svm](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/risc0/tests/direct-implementation/guest/src/svm5.rs)                    |  21156  |     0.028     |     0.028     |      2.351     |      0.602     |      0.002      |      0.002      |      2.381     |      0.632     |

</details>

#### RISC Zero Proofs of Proofs
<details open>
  <summary>RISC Zero (v0.16.1)</summary>

|      Examples<sup>*</sup>      |  Cycles | CPU Exec Time | GPU Exec Time | CPU Prove Time | GPU Prove Time | CPU Verify Time | GPU Verify Time | CPU Total Time | GPU Total Time |
|:-------------------:|:-------:|:-------------:|:-------------:|:--------------:|:--------------:|:---------------:|:---------------:|:--------------:|:--------------:|
| impreflex           |   66366 |     0.031     |     0.030     |       4.754    |      1.256     |      0.001      |      0.001      |       4.786    |      1.287     |
| transfer-goal       | 1139247 |     0.034     |     0.034     |      48.938    |     10.663     |      0.003      |      0.003      |      48.975    |     10.700     |
| batch-transfer-goal | 6724805 |     0.114     |     0.114     |     274.237    |     59.819     |      0.011      |      0.011      |     274.362    |     59.944     |
| perceptron-goal     | 3212346 |     0.049     |     0.050     |     127.911    |     28.433     |      0.006      |      0.006      |     127.966    |     28.489     |
| svm-goal            | 3212346 |     0.069     |     0.050     |     128.289    |     28.695     |      0.006      |      0.006      |     128.364    |     28.751     |

<sup>*</sup> The main implementation for RISC Zero $PI^2$ implementation is defined [here](https://github.com/Pi-Squared-Network/proof-checker-public/tree/master/risc0/pi2), and the inputs are defined [here](https://github.com/Pi-Squared-Network/proof-checker-public/tree/master/proofs/translated). The inputs are divided into three files: `*-gamma`, `*-claim`, and `*-proof`.
Ultimately, we anticipate that all $PI^2$ implementations will support a unique binary input format. As a result, all implementations will utilize the same inputs and have a single main implementation.
</details>

---

### [zkLLVM](https://github.com/NilFoundation/zkLLVM) Backend
#### zkLLVM Implementation Details

zkLLVM does not support GPU acceleration in any stage, so there are no GPU results for these experiments. 

The proof and verification for zkLLVM were generated using the command `transpiler -m gen-test-proof`.

The versions of the individual tools used for the examples are as follows:

```bash
$ clang-17 --version
clang version 17.0.4 (http://www.github.com/NilFoundation/zkllvm-circifier.git 4c393658e71bed430b996cff8555a548fbe8bbda)

$ assigner --version
0.1.11-48

$ transpiler --version
0.1.11-48
```

The `∞` symbol in the performance tables indicate that the example either did not finish executing after 6 hours or was terminated by the OS due to lack of memory.

#### zkLLVM Zero Direct Implementation
<details open>
  <summary>zkLLVM (v0.1.11-48)</summary>

|                                                  Examples                                                                          | CPU Circuit Gen Time | CPU Prove+Verify Time |
|:----------------------------------------------------------------------------------------------------------------------------------:|:--------------------:|:---------------------:|
| [transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/zkllvm/tests/direct-implementation/transfer)             |                0.730 |                 0.131 |
| [batch-transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/zkllvm/tests/direct-implementation/batch_transfer) |                1.367 |               143.183 |
| [perceptron](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/zkllvm/tests/direct-implementation/perceptron)         |                0.750 |                 0.130 |
| [svm](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/zkllvm/tests/direct-implementation/svm)                       |                0.730 |                 0.132 |

</details>

#### zkLLVM Proofs of Proofs
<details open> 
<summary>zkLLVM (v0.1.11-48)</summary>

  |       Examples      |CPU Circuit Gen Time | CPU Prove+Verify Time |
|:-------------------:|:-------------------:|:---------------------:|
| impreflex           |               0.585 |               298.686 |
| transfer-goal       |              31.585 |             24905.477 |
| batch-transfer-goal |             236.820 |                     ∞ |
| perceptron-goal     |              94.530 |                     ∞ |
| svm-goal            |              93.431 |                     ∞ |


The main implementation for the zkLLVM $PI^2$ implementation can be found [here](https://github.com/Pi-Squared-Network/proof-checker-public/tree/master/zkllvm/src). We translate the inputs, which are defined [here](https://github.com/Pi-Squared-Network/proof-checker-public/tree/master/proofs/translated). Binary inputs are divided and encoded into three arrays. Each file corresponds to the input requirements of the zkLLVM implementation.
</details>

---

### [Cairo](https://www.cairo-lang.org/) Backend
#### Cairo Direct Implementation

<details open>
<summary>Cairo Zero (v0.13.0)</summary>

|                                                             Examples                                                                            | CPU Exec Time | CPU Prove Time | CPU Verify Time | CPU Total Time |
|:------------------------------------------------------------------------------------------------------------------------------------------------|:-------------:|:--------------:|:---------------:|:--------------:|
| [transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo0/transfer.cairo)              |         0.440 |          0.195 |           0.008 |          0.643 |
| [batch-transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo0/batch_transfer.cairo)  |         6.825 |         30.196 |           0.869 |         37.890 |
| [perceptron](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo0/perceptron.cairo)          |         0.448 |          0.166 |           0.008 |          0.662 |
| [svm](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo0/svm.cairo)                        |         0.443 |          0.176 |           0.008 |          0.627 |


The programs were compiled using the default compiler of Cairo Zero, and were proven and verified using Lambdaworks Cairo Platinum Prover (v0.3.0)
</details>

<details open>
  <summary>Cairo One (v2.3.1)</summary>

  |                                                             Examples                                                                            | CPU Exec Time | CPU Prove Time | CPU Verify Time | CPU Total Time |
|:------------------------------------------------------------------------------------------------------------------------------------------------|:-------------:|:--------------:|:---------------:|:--------------:|
| [transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo1/transfer.cairo)              |         0.583 |          0.009 |           0.002 |          0.594 |
| [batch-transfer](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo1/batch_transfer.cairo)  |         0.691 |         65.693 |           1.787 |         68.171 |
| [perceptron](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo1/perceptron.cairo)          |         0.592 |          0.029 |           0.003 |          0.624 |
| [svm](https://github.com/Pi-Squared-Network/proof-checker-public/blob/master/cairo/tests/direct-implementation/cairo1/svm.cairo)                        |         0.593 |          0.032 |           0.003 |          0.628 |


The programs were compiled using LambdaClass Cairo-VM (v1.0.0-rc0), and were proven and verified using Lambdaworks Cairo Platinum Prover (v0.3.0)
</details>
