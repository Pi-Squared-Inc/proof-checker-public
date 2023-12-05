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
    echo "$1" | grep -E "Running execution \+ ZK certficate generation \+ verification" >> "$output_file"
    echo "-----------------------------------------------------------------------" >> "$output_file"
}

csl-example() {
    cd csl-examples
    echo "Running $1"
    cp guest/src/"$1".rs guest/src/main.rs
    output=$(cargo run --release)
    print_output "$output" "$1"
    cd ..
}

svm5() {
    csl-example "svm5"
}

perceptron() {
    csl-example "perceptron"
}

transfer() {
    csl-example "transfer"
}

transfer5000() {
    csl-example "transfer5000"
}

pi2-example() {
    echo "Running $1"
    gamma="$proofs_path/$1.ml-gamma"
    claim="$proofs_path/$1.ml-claim"
    proof="$proofs_path/$1.ml-proof"
    output=$(cargo run --release --bin host "$gamma" "$claim" "$proof")
    print_output "$output" "$1"
}

perceptron-goal() {
    pi2-example "perceptron-goal"
}

svm5-goal() {
    pi2-example "svm5-goal"
}

transfer-batch-1k-goal() {
    pi2-example "transfer-batch-1k-goal"
}

transfer-simple-compressed-goal() {
    pi2-example "transfer-simple-compressed-goal"
}

transfer-task-specific() {
    pi2-example "transfer-task-specific"
}

impreflex-compressed-goal() {
    pi2-example "impreflex-compressed-goal"
}

clean_up() {
    if [ -f "csl-examples/guest/src/main.rs" ]; then
        mv csl-examples/"$output_file" "$output_file"
        rm csl-examples/guest/src/main.rs
    fi
}

csl() {
    echo "Running all examples"
    svm5
    perceptron
    transfer
    transfer5000
}

pi2() {
    echo "Running pi2"
    perceptron-goal
    svm5-goal
    transfer-batch-1k-goal
    transfer-simple-compressed-goal
    transfer-task-specific
    impreflex-compressed-goal
}

all() {
    csl
    clean_up
    pi2
}

if [ -z "$2" ]; then
    all
else
    "$2"
fi

clean_up