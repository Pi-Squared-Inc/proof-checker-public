#!/bin/bash
# Usage: check_with_risc0.sh [options] <input directory>
#
# Options:
#  -a, --available            Print if the program is available or not
#  -h, --help                 Print this help message
#  -l, --log FILENAME         Save the output log to a file
#  -s, --stats                Print statistics about the input
#  -v, --verbose              Print the output of the execution

BASH_SOURCE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
BUILD_DIR="$BASH_SOURCE_DIR/../.build"


# Parse arguments
while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        -a|--available)
            if [ -f "$BUILD_DIR/target/release/host" ]; then
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
            echo " -s, --stats             Print statistics about the execution"
            echo " -v, --verbose           Print the output of the execution"
            exit 0
            ;;
        -o|--log)
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

# Check if the input directory contains the input files necessaries for RISC0
input_filename=$(basename "$input")
if [ ! -f "$input/$input_filename.ml-gamma" ]; then
    echo "Input directory does not contain a $input_filename.ml-gamma file"
    exit 1
fi

if [ ! -f "$input/$input_filename.ml-claim" ]; then
    echo "Input directory does not contain a $input_filename.ml-claim file"
    exit 1
fi

if [ ! -f "$input/$input_filename.ml-proof" ]; then
    echo "Input directory does not contain a $input_filename.ml-proof file"
    exit 1
fi

# Setting the inputs variable
gamma="$input/$input_filename.ml-gamma"
claim="$input/$input_filename.ml-claim"
proof="$input/$input_filename.ml-proof"

# If $log_file is not set, then we will print the output to stdout
if [ -z "$log_file" ]; then
    log_file="/dev/stdout"
fi

# Execute the RISC0 Proof-Checker with the input files
if [ "$stats" ]; then
   output=$(cargo run --release --bin host "$gamma" "$claim" "$proof")
   if [ "$verbose" ]; then
       echo "$output"
   fi
   {
        echo "$input_filename"
        echo "$output" | grep -E "Total cycles"
        echo "$output" | grep -E "Ran in "
        echo "$output" | grep -E "Proved in "
        echo "$output" | grep -E "Verified in "
        echo "$output" | grep -E "Running execution \+ ZK certficate generation \+ verification"
        echo "-----------------------------------------------------------------------"
   } >> "$log_file"
else
    cargo run --release --bin host "$gamma" "$claim" "$proof" >> /dev/null
fi

exit 0;
