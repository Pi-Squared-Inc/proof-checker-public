#!/bin/bash
# Usage: bench.sh <output file> [example name]

if [ -z "$1" ]; then
    echo "Usage: bench.sh <output file> [example name]"
    exit 1
fi

output_file="$1"

current_dir=$(pwd)
script_dir=$(dirname "$0")
proofs_path="${script_dir}/../.build/proofs/translated"

direct-example() {
    (
        cd tests/direct-implementation || exit
        echo "Running $1"
        cp guest/src/"$1".rs guest/src/main.rs
        output=$(cargo run --release)
        print_output "$output" "$1"
    )
}

svm() {
    direct-example "svm5"
}

perceptron() {
    direct-example "perceptron"
}

transfer() {
    direct-example "transfer"
}

batch-transfer-csl() {
    direct-example "transfer5000"
}

perceptron-goal() {
    ./check_with_risc0.sh --stats -o "$output_file" "$proofs_path/perceptron-goal"
}

svm-goal() {
    ./check_with_risc0.sh --stats -o "$output_file" "$proofs_path/svm5-goal"
}

batch-transfer() {
    ./check_with_risc0.sh --stats -o "$output_file" "$proofs_path/transfer-batch-1k-goal"
}

transfer-goal() {
    ./check_with_risc0.sh --stats -o "$output_file" "$proofs_path/transfer-simple-compressed-goal"
}

impreflex() {
    ./check_with_risc0.sh --stats -o "$output_file" "$proofs_path/impreflex-compressed-goal"
}

clean_up() {
    if [ -f "tests/direct-implementation/guest/src/main.rs" ]; then
        mv tests/direct-implementation/"$output_file" "$output_file"
        rm tests/direct-implementation/guest/src/main.rs
    fi
}

direct() {
    echo "Running all examples"
    transfer
    batch-transfer-csl
    perceptron
    svm
}

pi2() {
    cd "${script_dir}/.." || exit
    make test-proof-translate-bin
    cd "${current_dir}" || exit

    echo "Running pi2"
    impreflex
    transfer-goal
    batch-transfer
    perceptron-goal
    svm-goal
}

all() {
    direct
    clean_up
    pi2
}

if [ -z "$2" ]; then
    all
else
    "$2"
fi

clean_up