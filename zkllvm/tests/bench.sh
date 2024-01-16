#!/bin/bash
# Usage: bench.sh <output file> [example name]

if [ -z "$1" ]; then
    echo "Usage: bench.sh <output file> [example name]"
    exit 1
fi

output_file="../$1"
build_script="../../check_with_zkllvm.sh"

print_output() {
    { 
        echo "Executing $2"
        echo "$1" | grep -E "$2"
        echo "-----------------------------------------------------------------"
    } >> "$output_file"
}

direct-implementation() {
    (
        cd direct-implementation || exit
        echo "Running $1"
        output=$($build_script --stats --transpiler --translate-input-off "$1/$3" "$1/$2")
        print_output "$output" "$1"
    )
}

transfer() {
    direct-implementation "transfer" "transfer_test.cpp" "transfer_input.json"
}

batch-transfer-csl() {
    direct-implementation "batch_transfer" "batch_transfer_test.cpp" "batch_transfer_input.json"
}

perceptron() {
    direct-implementation "perceptron" "perceptron_test.cpp" "perceptron_input.json"
}

svm() {
    direct-implementation "svm" "svm_test.cpp" "svm_input.json"
}

pi2-example() {
    (
        cd proofs-of-proofs || exit
        echo "Running $1"
        output=$($build_script --stats --transpiler --translate-input-off "$1.inp")
        print_output "$output" "$1"
    )
}

impreflex() {
    pi2-example "impreflex"
}

transfer-goal() {
    pi2-example "transfer-goal"
}

batch-transfer() {
    pi2-example "batch-transfer-goal"
}

perceptron-goal() {
    pi2-example "perceptron-goal"
}

svm-goal() {
    pi2-example "svm-goal"
}

direct() {
    echo "Running Direct Implementation CSL Examples"
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
