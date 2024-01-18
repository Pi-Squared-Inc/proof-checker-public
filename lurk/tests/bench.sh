#!/bin/bash
# Usage: bench.sh <output file> [example name]

if [ -z "$1" ]; then
    echo "Usage: bench.sh <output file> <cpu|gpu> [example name]"
    exit 1
fi

output_file="../$1"
mode=
if [ -n "$2" ]; then
    if [ "$2" == "cpu" ]; then
        mode="--cpu"
    else
        mode="--gpu"
    fi
else
    echo "Usage: bench.sh <output file> <cpu|gpu> [example name]"
    exit 1
fi

build_script="../../check_with_lurk.sh"

print_output() {
    { 
        echo "Executing $2"
        echo "$1"
        echo "-----------------------------------------------------------------"
    } >> "$output_file"
}

direct-implementation() {
    (
        cd direct-implementation || exit
        echo "Running $1"
        output=$($build_script --rc "$2" --stats $mode --translate-input-off "$1.lurk")
        print_output "$output" "$1"
    )
}

transfer() {
    direct-implementation "transfer" 10
}

batch-transfer-csl() {
    direct-implementation "batch_transfer" 400
}

perceptron() {
    direct-implementation "perceptron" 10
}

svm() {
    direct-implementation "svm" 10
}

pi2-example() {
    (
        cd proofs-of-proofs || exit
        echo "Running $1"
        output=$($build_script --stats "$mode" --translate-input-off "test_$1.lurk")
        print_output "$output" "$1"
    )
}

impreflex() {
    pi2-example "impreflex"
}

transfer-goal() {
    pi2-example "transfer_goal"
}

batch-transfer() {
    pi2-example "batch_transfer_goal"
}

perceptron-goal() {
    pi2-example "perceptron_goal"
}

svm-goal() {
    pi2-example "svm_goal"
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
    pi2
}

if [ -z "$3" ]; then
    all
else
    "$3"
fi

# Clean up
rm -r ~/.lurk/proofs
