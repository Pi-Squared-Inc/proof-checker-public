#!/bin/bash
# Usage: check_with_risc0.sh [options] <input directory>
#
# Options:
#  -a, --available            Print if the program is available or not
#  -h, --help                 Print this help message
#  -l, --log FILENAME         Save the output log to a file
#  -o, --output FILENAME      Save the output to a file
#  -s, --stats                Print statistics about the input
#  -v, --verbose              Print the output of the execution
#      --version              Print the Proof Version accepted by RISC0

BASH_SOURCE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
BUILD_DIR="$BASH_SOURCE_DIR/../.build"
pi2_risc0_version=$(cargo pkgid --manifest-path "$BASH_SOURCE_DIR/prover/Cargo.toml" | cut -d "@" -f2)

# Parse arguments
while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        -a|--available)
            if [[ -f "$BUILD_DIR/target/release/prover" && -f "$BUILD_DIR/target/release/verifier" ]]; then
                echo "1"
            else
                echo "0"
            fi
            exit 0
            ;;
        -h|--help)
            echo "Usage: check_with_risc0.sh [options] <input directory>"
            echo ""
            echo "Options:"
            echo " -a, --available         Print if the program is available or not"
            echo " -h, --help              Print this help message"
            echo " -l, --log FILENAME      Save the output log to a file"
            echo " -0, --output FILENAME   Save the output to a file"
            echo " -s, --stats             Print statistics about the execution"
            echo " -v, --verbose           Print the output of the execution"
            echo "     --version           Print the Proof Version accepted by RISC0"
            exit 0
            ;;
        -o|--output)
            output_file="$2"
            shift
            shift
            ;;
        -l|--log)
            log_file="$2"
            shift
            shift
            ;;
        -s|--stats)
            stats=true
            shift
            ;;
        -v|--verbose)
            verbose=true
            shift
            ;;
        --version)
            echo "$pi2_risc0_version"
            exit 0
            ;;
        *)
            input="$1"
            shift
            ;;
    esac
done

if [ -z "$input" ]; then
    echo "Usage: check_with_risc0.sh [options] <input directory>"
    exit 1
fi

# Check if the input directory exists
if [ ! -d "$input" ]; then
    echo "Input directory does not exist"
    exit 1
fi

# Check if the version accepted by RISC0 is the same as the version of the proof
proof_generation_version=$(poetry -C "$BASH_SOURCE_DIR/../generation" run python -c "from importlib.metadata import version; print(version('proof_generation'))" )
if [ "$pi2_risc0_version" != "$proof_generation_version" ]; then
    echo "Error: The version of the proof ($proof_generation_version) is different from the version accepted by RISC0 ($pi2_risc0_version)"
    exit 1
fi

target_files=true
proof_files=true

# Check if the input directory contains the input files necessaries for RISC0
input_filename=$(basename "$input")
if [ ! -f "$input/$input_filename.target.ml-gamma" ]; then
    #echo "Input directory does not contain a $input_filename.target.ml-gamma file"
    #exit 1
    target_files=false
fi

if [ ! -f "$input/$input_filename.target.ml-claim" ]; then
    # echo "Input directory does not contain a $input_filename.target.ml-claim file"
    # exit 1
    target_files=false
fi

# Check if the input directory contains the input files necessaries for RISC0
input_filename=$(basename "$input")
if [ ! -f "$input/$input_filename.ml-gamma" ]; then
    # echo "Input directory does not contain a $input_filename.ml-gamma file"
    # exit 1
    proof_files=false
fi

if [ ! -f "$input/$input_filename.ml-claim" ]; then
    # echo "Input directory does not contain a $input_filename.ml-claim file"
    # exit 1
    proof_files=false
fi

if [ ! -f "$input/$input_filename.ml-proof" ]; then
    # echo "Input directory does not contain a $input_filename.ml-proof file"
    # exit 1
    proof_files=false
fi

if [ "$target_files" = false ] && [ "$proof_files" = false ]; then
    echo "Input directory does not contain a $input_filename.ml-gamma, $input_filename.ml-claim, $input_filename.ml-proof, $input_filename.target.ml-gamma, or $input_filename.target.ml-claim file"
    exit 1
fi

if [ "$target_files" = false ] && [ "$proof_files" = true ]; then
    cp "$input/$input_filename.ml-gamma" "$input/$input_filename.target.ml-gamma"
    cp "$input/$input_filename.ml-claim" "$input/$input_filename.target.ml-claim"
fi

# Setting the inputs variable
gamma_target="$input/$input_filename.target.ml-gamma"
claim_target="$input/$input_filename.target.ml-claim"

# If $log_file is not set, then we will print the output to stdout
if [ -z "$log_file" ]; then
    log_file="/dev/stdout"
fi

# If $output_file is not set, then we will print the output to stdout
if [ -z "$output_file" ]; then
    output_file="$input_filename.receipt"
fi

# Execute the RISC0 Proof-Checker Prover with the input files
echo "Running RISC0 Prover"

if [ "$stats" ]; then
   output=$(cargo run --release --bin prover "$gamma_target" "$claim_target" "$input" "$output_file")
   {
        echo "$input_filename"
        echo "$output" | grep -E "Proof Checker and SDC receipts written to "
        echo "$output" | grep -E "Running and proving all proof-checker executions \+ ZK certficates generation "
        echo "$output" | grep -E "Running and proving sdc execution \+ ZK certficate generation "
        echo "$output" | grep -E "Total time executing and proving took "
        echo "-----------------------------------------------------------------------"
   } >> "$log_file"
else
    output=$(cargo run --release --bin prover "$gamma_target" "$claim_target" "$input" "$output_file")
fi

if [ "$verbose" ]; then
   echo "$output"
fi

# Execute RISC0 Proof-Checker Verifier with the input and output files
echo "Running RISC0 Verifier"

if [ "$stats" ]; then
   output=$(cargo run --release --bin verifier "$gamma_target" "$claim_target" "$output_file")
   {
        echo "$input_filename"
        echo "$output" | grep -E "Sum of Total cycles "
        echo "$output" | grep -E "Sum of Verification time  "
        echo "$output" | grep -E "SDC and Proof Checker was verified "
        echo "-----------------------------------------------------------------------"
   } >> "$log_file"
else
    output=$(cargo run --release --bin verifier "$gamma_target" "$claim_target" "$output_file")
fi

if [ "$verbose" ]; then
   echo "$output"
fi

exit 0;
