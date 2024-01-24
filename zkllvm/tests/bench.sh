#!/bin/bash
# Usage: bench.sh <output file> [example name]

if [ -z "$1" ]; then
    echo "Usage: bench.sh <output file> [example name]"
    exit 1
fi

bash_source_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
output_file="$bash_source_dir/$1"
build_script="$bash_source_dir/../check_with_zkllvm.sh"
pi2_proofs_dir="$bash_source_dir/../../proofs/translated"

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
        echo "Running $1"
        output=$($build_script --stats --transpiler "$pi2_proofs_dir/$1")
        print_output "$output" "$1"
    )
}

impreflex() {
    pi2-example "impreflex-compressed-goal"
}

transfer-goal() {
    pi2-example "transfer-simple-compressed-goal"
}

batch-transfer() {
    pi2-example "transfer-batch-1k-goal"
}

perceptron-goal() {
    pi2-example "perceptron-goal"
}

svm-goal() {
    pi2-example "svm5-goal"
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
