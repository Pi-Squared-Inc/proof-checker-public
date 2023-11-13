# zkLLVM implementation for Proof-Checker

## Setting up zkLLVM
There is a list of dependencies that need to be installed before zkLLVM can be built:
```bash
sudo apt install build-essential libssl-dev cmake clang-12 git curl pkg-config libspdlog-dev
```

Note that zkLLVM requires the boost lib version to be greater than 1.76.0.
For Unix based system, we can download the source code from [boost](https://www.boost.org) website
and run the following command under the source code directory to install it:
```bash
sudo ./bootstrap.sh --prefix=/usr/
sudo ./b2 install
```

For the complete list of dependencies, please refer to the
[zkllvm#install-dependencies](https://github.com/NilFoundation/zkllvm#install-dependencies).

After installing the dependencies, we can clone and build zkLLVM:
```bash
git clone --recurse-submodules git@github.com:NilFoundation/zkllvm.git
cd zkllvm
```
=Nil; Foundation uses "Unix Makefiles" as a CMake generator. Personally, I use
"Ninja" as a CMake generator, as I have experienced faster builds with it,
especially with increment builds. Both are fine.
Install Ninja dependencie.
```bash
sudo apt-get install ninja-build
```

Then, we can finally configure the CMake and build the zkLLVM compiler:
```bash
cmake -G "Ninja" -B ${ZKLLVM_BUILD:-build} -DCMAKE_BUILD_TYPE=Release -DCIRCUIT_ASSEMBLY_OUTPUT=TRUE .
ninja -C ${ZKLLVM_BUILD:-build} assigner clang llvm-link zkllvm-libc transpiler -j$(nproc) 
```
`clang` is the zkLLVM compiler, and `assigner` is the tool that assigns the
input to the circuit and can tell us if the circuit is satisfiable or not.
From their official documentation: 
> You can also run the assigner with the --check flag to validate the
satisfiability of the circuit. If the circuit is satisfiable, the assigner will
output the satisfying assignment in the assignment.tbl file. If there is an
error, the assigner will output the error message and throw an exception via
std::abort.

Further reading for zkLLVM usage: [zkllvm#usage](https://github.com/NilFoundation/zkllvm#usage).

## Compiling the Proof-Checker
The Proof-Checker is written in C++ and uses CMake as a build system. 
The following commands will build the Proof-Checker:
```bash
# TODO: Add instructions for building the Proof-Checker with CMake
```

### We also have a temporary bash script that builds and execute the Proof-Checker:

First you need to set the environment variable `ZKLLVM_ROOT`:
```bash
echo 'export ZKLLVM_ROOT=/path/to/zkllvm' >> ~/.zshrc
source ~/.zshrc
```

Then you can build and execute the Proof-Checker with real input:
```bash
./build.sh src/main.cpp inputs/impreflex-compressed-goal.inp
```

You can also run it with g++/clang to any textual output or exception that may happen:
```bash
g++ -DDEBUG=1 -std=c++20 -g src/main.cpp -o ../.build/zkllvm/a.out && ../.build/zkllvm/a.out 
```

If you want to run the Proof-Checker unit tests, you can use the followinf commands:
For zkLLVM:
```bash
./build.sh src/tests.cpp inputs/example.inp
```

For g++/clang:
```bash
g++ -DDEBUG=1 -std=c++20 -g src/tests.cpp -o ../.build/zkllvm/a.out && ../.build/zkllvm/a.out
```

### Translation from binary input to zkllvm input
We need to translate the binary input to a JSON format that zkllvm accepts.
The zkllvm input is an array that contains three vectors. The vectors represent assumption, claim and proof.
Each vector will have a fixed size. The first element is the actual length.
It is followed by the actual input that is an integer array translated from the binary input.
The rest of the vector is padded by NO-OP instruction till the fixed size is reached.

We have a python script translating the binary input:
```bash
python3 translator.py <path-to-assumption> <path-to-claim> <path-to-proof>
```

For example,
```bash
python3 translator.py ../proofs/translated/impreflex-compressed-goal.ml-gamma ../proofs/translated/impreflex-compressed-goal.ml-claim ../proofs/translated/impreflex-compressed-goal.ml-proof > inputs/impreflex-compressed-goal.inp
```
