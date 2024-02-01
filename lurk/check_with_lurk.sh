#!/bin/bash
# Usage: check_with_lurk.sh [options] <input directory>
#
# Options:
#  -a,  --available            Print if Lurk is available or not
#  -h,  --help                 Print this help message
#  -l,  --log FILENAME         Save the output to a file
#  -s,  --stats                Print statistics about the execution
#  -v,  --verbose              Print the output of the program
#  -vv, --very-verbose         Print the output of the program and set -x
#       --cpu                  Use the CPU version of the program
#       --gpu                  Use the GPU version of the program [default if available]
#       --no-verify            Do not verify the proof
#       --rc NUMBER            Set the number of iterations packed in a batch [default: 400]
#       --translate-input-off  Turn off the translation of the input files and 
#                              assume that the input is a JSON file
#       --version              Print the Proof Version accepted by Lurk

# This version should always be in sync with the proof that will be the input
# When changing this version, remember to change the version in:
# proof-checker/lurk/src/lib.lurk
pi2_lurk_version="0.1.0"

# Default values
cpu=true
log="/dev/stdout"
stats=false
verify=true
translate_input=true
rc=400
script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Parse the arguments
while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        -a|--available)
             if command -v lurk &> /dev/null; then
                echo "1"
            else
                echo "0"
            fi
            exit 0
            ;;
        -h|--help)
            echo "Usage: check_with_lurk.sh [options] <input directory>"
            echo ""
            echo "Options:"
            echo " -a,  --available            Print if Lurk is available or not"
            echo " -h,  --help                 Print this help message"
            echo " -l,  --log FILENAME         Save the output to a file"
            echo " -s,  --stats                Print statistics about the execution"
            echo " -v,  --verbose              Print the output of the program"
            echo " -vv, --very-verbose         Print the output of the program and set -x"
            echo "      --cpu                  Use the CPU version of the program"
            echo "      --gpu                  Use the GPU version of the program [default if available]"
            echo "      --no-verify            Do not verify the proof"
            echo "      --rc NUMBER            Set the number of iterations packed in a batch [default: 400]"
            echo "      --translate-input-off  Turn off the translation of the input files and"
            echo "                             assume that the input is a JSON file"
            echo "      --version              Print the Proof Version accepted by Lurk"
            exit 0
            ;;
        -l|--log)
            log="$2"
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
        -vv|--very-verbose)
            very_verbose=true
            shift
            ;;
        --cpu)
            cpu=true
            shift
            ;;
        --gpu)
            gpu=true
            shift
            ;;
        --no-verify)
            verify=false
            shift
            ;;
        --rc)
            rc="$2"
            shift
            shift
            ;;
        --translate-input-off)
            translate_input=false
            shift
            ;;
        --version)
            echo "$pi2_lurk_version"
            exit 0
            ;;
        *)
            input="$1"
            shift
            ;;
    esac
done

if [ -z "$input" ]; then
    echo "Usage: check_with_lurk.sh [options] <input directory>"
    exit 1
fi

# Check if Lurk is available
if ! command -v lurk &> /dev/null; then
    echo "Error: Lurk is not available." \
         "Please install Lurk-rs and make sure that the binary is in your PATH."
    exit 1
fi

# If very_verbose is set, then set -x
if [ "$very_verbose" == true ]; then
    set -x
fi

# Check if the version accepted by Lurk is the same as the version of the proof
proof_generation_version=$(poetry -C "$script_dir/../generation" run python -c "from importlib.metadata import version; print(version('proof_generation'))" )
if [ "$pi2_lurk_version" != "$proof_generation_version" ]; then
    echo "Error: The version of the proof ($proof_generation_version) is different from the version accepted by Lurk ($pi2_lurk_version)"
    exit 1
fi

# Translate the binary files into the unique input JSON file accepted by zkLLVM
# Check if the input directory contains the input files necessaries for the translator
input_filename=$(basename "$input")
if [ "$translate_input" == true ]; then 
    # Check if the input directory exists
    if [ ! -d "$input" ]; then
        echo "Input directory does not exist"
        exit 1
    fi

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

     lib_lurk="$script_dir/src/lib.lurk"

     # Check if the lib.lurk file exists
    if [ ! -f "$lib_lurk" ]; then
        echo "lib.lurk file does not exist"
        exit 1
    fi

     # Execute the translator with the input files and save the output to a temporary file
     tmp_input_file=$(mktemp)
     "$script_dir"/translator.py "$gamma" "$claim" "$proof" "$lib_lurk" > "$tmp_input_file"
else
    # Check if the input file exists
    if [ ! -f "$input" ]; then
        echo "Input file \"$input\" does not exist"
        exit 1
    fi

    tmp_input_file="$input"
fi

# Check if nvcc is available, if yes, then use the GPU version
if command -v nvcc &> /dev/null; then
    cpu=false
fi

if [ "$gpu" == true ]; then
    if ! command -v nvcc &> /dev/null; then
        echo "Error: nvcc is not available." \
             "Please install CUDA and make sure that the binary is in your PATH."
        exit 1
    fi
    cpu=false
fi


if [ "$cpu" == true ]; then
    export CUDA_PATH=
    export NVCC=off
    export EC_GPU_FRAMEWORK=none
fi

if [ "$stats" == false ]; then
    prove_output=$(lurk --rc "$rc" "$tmp_input_file")
    proof_key=$(echo "$prove_output" | grep -E "Proof key:" | sed -E 's/Proof key: \"(.*)\"/\1/')
    [ "$verify" == true ] && verify_output=$(lurk verify "$proof_key")

    if [ "$verbose" == true ]; then
        {
            echo "$prove_output"
            [ "$verify" == true ] && echo "$verify_output"
        } >> "$log"
    fi
else
    START_PROVE_TIME=$(date +%s%3N)
    prove_output=$(lurk --rc "$rc" "$tmp_input_file")
    END_PROVE_TIME=$(date +%s%3N)

    proof_key=$(echo "$prove_output" | grep -E "Proof key:" | sed -E 's/Proof key: \"(.*)\"/\1/')
    iterations=$(echo "$prove_output" | grep -E "\[[0-9]* iterations\]" | sed -E 's/\[([0-9]*) iterations\] => .*/\1/')

    TOTAL_PROVE_TIME=$(("$END_PROVE_TIME" - "$START_PROVE_TIME"))

    if [ "$verify" == true ]; then
        START_VERIFY_TIME=$(date +%s%3N)
        verify_output=$(lurk verify "$proof_key")
        END_VERIFY_TIME=$(date +%s%3N)

        TOTAL_VERIFY_TIME=$(("$END_VERIFY_TIME" - "$START_VERIFY_TIME"))
    fi

    {   
        [ "$verbose" == true ] && echo "$prove_output" && echo ""
        echo "Iterations: $iterations"
        echo "Proving $input_filename in" $(("$TOTAL_PROVE_TIME" / 1000)).$(("$TOTAL_PROVE_TIME" % 1000)) "s"
        if [ "$verify" == true ]; then
            echo "Verifying $input_filename" "in" $(("$TOTAL_VERIFY_TIME" / 1000)).$(("$TOTAL_VERIFY_TIME" % 1000)) "s"
            TOTAL=$(("$TOTAL_PROVE_TIME" + "$TOTAL_VERIFY_TIME"))
            echo "Total time" $(("$TOTAL" / 1000)).$(("$TOTAL" % 1000)) "s"
        fi
        echo ""
        [ "$verify" == true ] && [ "$verbose" == true ] && echo "$verify_output"
    } >> "$log"
fi
