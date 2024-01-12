#!/bin/bash
# Usage: bench.sh <output file> [example name]

if [ -z "$1" ]; then
    echo "Usage: bench.sh <output file> [example name]"
    exit 1
fi

output_file="$1"
proofs_path="../proofs/translated"


print_output() {
    echo "$2" >> "$output_file"
    echo "$1" | grep -E "Total cycles" >> "$output_file"
    echo "$1" | grep -E "Ran in " >> "$output_file"
    echo "$1" | grep -E "Proved in " >> "$output_file"
    echo "$1" | grep -E "Verified in " >> "$output_file"
    echo "$1" | grep -E "Running execution \+ ZK certficate generation \+ verification" >> "$output_file"
    echo "-----------------------------------------------------------------------" >> "$output_file"
}

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

pi2-example() {
    echo "Running $1"
    gamma="$proofs_path/$1/$1.ml-gamma"
    claim="$proofs_path/$1/$1.ml-claim"
    proof="$proofs_path/$1/$1.ml-proof"
    output=$(cargo run --release --bin host "$gamma" "$claim" "$proof")
    print_output "$output" "$1"
}

perceptron-goal() {
    pi2-example "perceptron-goal"
}

svm-goal() {
    pi2-example "svm5-goal"
}

batch-transfer() {
    pi2-example "transfer-batch-1k-goal"
}

transfer-goal() {
    pi2-example "transfer-simple-compressed-goal"
}

impreflex() {
    pi2-example "impreflex-compressed-goal"
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