#!/bin/bash
# Usage: check_with_zkllvm.sh [options] <input directory>
#
# Options:
#  -a,  --available            Print if the program is available or not
#  -h,  --help                 Print this help message
#  -l,  --log FILENAME         Save the output to a file
#  -o,  --output DIRNAME       Save the output files to a directory
#  -pp, --proof-producer       Use the proof producer to proof and verify.
#                              THREAD_MODE can be 'single' or 'multi'
#  -s,  --stats                Print the time spent of the assigner and transpiler
#  -t,  --transpiler           [Deprecated] Use the transpiler to proof and verify
#  -ti, --translate-input-off  Turn off the translation of the input files and
#                              assume that the input is a JSON file
#  -v,  --verbose              Print the output of the assigner and transpiler
#  -vv, --very-verbose         Print the versbose execution of this script
#       --version              Print the Proof Version accepted by zkLLVM

# This version should always be in sync with the proof that will be the input
# When changing this version, remember to change the version in:
# proof-checker/cpp/src/lib.hpp
pi2_zkllvm_version="0.1.0"

bash_source_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
build_dir="$bash_source_dir/../.build"

transpiler=false
proof_producer=true
translate_input=true
PROOF_PRODUCER_BIN="proof-generator-single-threaded"
PROOF_PRODUCER_MODE=single-threaded

assigner_is_available() {
    output=${1:-/dev/null}
    if command -v assigner &> /dev/null; then
        return 0
    else
        echo "Error: Assigner is not installed." \
             "Please install the assigner from zkLLVM and include it in your PATH." >&"$output"
        return 1
    fi
}

transpiler_is_available() {
    output=${1:-/dev/null}
    if command -v transpiler &> /dev/null; then
        return 0
    else
        echo "Error: Transpiler is not installed." \
             "Please install the transpiler from zkLLVM and include it in your PATH." >&"$output"
        return 1
    fi
}

proof_producer_is_available() {
    output=${1:-/dev/null}
    if command -v proof-generator-single-threaded &> /dev/null; then
        return 0
    else
        if command -v proof-generator-multi-threaded &> /dev/null; then
            return 0
        else
             echo "Error: Proof-Producer is not installed." \
                  "Please install the proof-producer from proof-producer and include it in your PATH." >&"$output"
            return 1
        fi
    fi
}

clean_up() {
    rm "$OUTPUT_TABLE" 2> /dev/null
    rm "$1/circuit_params.json" 2> /dev/null
    rm "$OUTPUT_CIRCUIT" 2> /dev/null
    rm "$OUTPUT_CLANG" 2> /dev/null
    rm "$OUTPUT_LLVM_LINK_1" 2> /dev/null
    rm "$OUTPUT_LLVM_LINK_2" 2> /dev/null
    rm "$1/proof.bin" 2> /dev/null
    rm "$1/public_input.bin" 2> /dev/null
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        -a|--available)
            if assigner_is_available && (transpiler_is_available || proof_producer_is_available); then
                echo "1"
            else
                echo "0"
            fi
            exit 0
            ;;
        -h|--help)
            echo "Usage: check_with_zkllvm.sh [options] <input directory>"
            echo ""
            echo "Options:"
            echo " -a,  --available                    Print if the program is available or not"
            echo " -h,  --help                         Print this help message"
            echo " -l,  --log FILENAME                 Save the output to a file"
            echo " -o,  --output DIRNAME               Save the output files to a directory"
            echo " -pp, --proof-producer THREAD_MODE   Use the proof producer to proof and verify. THREAD_MODE can be 'single' or 'multi'"
            echo " -s,  --stats                        Print statistics about the execution"
            echo " -t,  --transpiler                   [Deprecated] Use the transpiler to proof and verify"
            echo " -ti, --translate-input              Turn off the translation of the input files and assume that the input is a JSON file"
            echo " -v,  --verbose                      Print the output of the assigner and transpiler"
            echo "      --version                      Print the Proof Version accepted by zkLLVM"
            exit 0
            ;;
        -l|--log)
            log_file="$2"
            shift
            shift
            ;;
        -o|--output)
            output_dir="$2"
            shift
            shift
            ;;
        -pp|--proof-producer)
            proof_producer=true
            transpiler=false
            if [ "$2" == "single" ]; then
                PROOF_PRODUCER_BIN="proof-generator-single-threaded"
                PROOF_PRODUCER_MODE=single-threaded
            elif [ "$2" == "multi" ]; then
                PROOF_PRODUCER_BIN="proof-generator-multi-threaded"
                PROOF_PRODUCER_MODE=multi-threaded
            else
                echo "Error: Invalid argument for --proof-producer. Valid arguments are 'single' or 'multi'"
                exit 1
            fi
            shift
            shift
            ;;
        -s|--stats)
            stats=true
            shift
            ;;
        -t|--transpiler)
            transpiler=true
            proof_producer=false
            echo "Use transpiler to generate the proof and verify it is" \
                 "deprecated and will be removed in the future. Please consider" \
                 "use the proof-producer instead."
            shift
            ;;
        -ti|--translate-input-off)
            translate_input=false
            shift
            ;;
        -v|--verbose)
            verbose=true
            shift
            ;;
        -vv|--very-verbose)
            verbose=true
            very_verbose=true
            set -x
            shift
            ;;
        --version)
            echo "$pi2_zkllvm_version"
            exit 0
            ;;
        *)
            input="$1"
            shift

            # If $2 is not set, then we will use the default proof-checker main source code
            # having this is useful for testing purposes.
            if [ -z "$1" ]; then
                main_source="$bash_source_dir/src/main.cpp"
            else
                main_source="$1"
            fi
            shift
            ;;
    esac
done

# Check if ZKLLVM_ROOT is set, as it is necessary for the script to work
if [[ -z "${ZKLLVM_ROOT}" ]]; then
  echo "ZKLLVM_ROOT is not set"
  exit 1
fi

if [ -z "$input" ]; then
    echo "Usage: check_with_zkllvm.sh [options] <input directory>"
    exit 1
fi

# if verbose is set, then set -x
if [ "$very_verbose" ]; then
    set -x
fi

# Check if the version accepted by zkLLVM is the same as the version of the proof
proof_generation_version=$(poetry -C "$bash_source_dir/../generation" run python -c "from importlib.metadata import version; print(version('proof_generation'))" )
if [ "$pi2_zkllvm_version" != "$proof_generation_version" ] && [ "$translate_input" = true ]; then
    echo "Error: The version of the proof ($proof_generation_version) is different from the version accepted by zkLLVM ($pi2_zkllvm_version)"
    exit 1
fi

# Translate the binary files into the unique input JSON file accepted by zkLLVM
# Check if the input directory contains the input files necessaries for the translator
input_filename=$(basename "$input")
alen=27001
clen=27001
plen=27001
maxlen=27001
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

    # Execute the translator with the input files and save the output to a temporary file
    tmp_input_file=$(mktemp)

    result=$(python3 "$bash_source_dir/translator.py" "$input" -o "$tmp_input_file")

    IFS=', ' read -r -a arr <<< "$result"
    alen=${arr[0]}
    clen=${arr[1]}
    plen=${arr[2]}
    maxlen=${arr[3]}
else
    if [ ! -f "$input" ]; then
        echo "Input directory does not exist"
        exit 1
    fi

    tmp_input_file="$input"
    input_filename=${input_filename%%.*}
fi

# Creating output directory under `.build` if it doesn't exist or clean it if it does
if [[ ! -d "${build_dir}" ]]; then
  mkdir "${build_dir}"
fi
if [[ ! -d "${build_dir}/zkllvm" ]]; then
  mkdir "${build_dir}/zkllvm"
fi


main_file=$(basename "$main_source")
main_file_path=$(dirname "$(realpath "$main_source")")
main_file_name=${main_file%%.*}

# Creating output directory under `.build` if it doesn't exist or clean it if it does
if [[ -z "$output_dir" ]]; then
  output_dir="$build_dir/zkllvm/$main_file_name"
fi

if [[ ! -d "$output_dir" ]]; then
  mkdir "$output_dir"
else
  clean_up "$output_dir"
fi

crypto3_lib_dir="$ZKLLVM_ROOT/libs/crypto3"
clang="${ZKLLVM_ROOT}"/build/libs/circifier/llvm/bin/clang-16
llvm_link="${ZKLLVM_ROOT}"/build/libs/circifier/llvm/bin/llvm-link
LIB_C="${ZKLLVM_ROOT}/build/libs/stdlib/libc"

# if clang and libc doesn't exist, then exit
if [ ! -f "$clang" ]; then
    echo "Error: clang-17 is not installed. Please install the circifier from zkLLVM and include it in your PATH."
    exit 1
fi

if [ ! -d "$LIB_C" ]; then
    echo "Error: libc is not installed. Please install the circifier from zkLLVM and include it in your PATH."
    exit 1
fi


# Intermediary outputs:
OUTPUT_CLANG="$output_dir/${main_file_name}_cpp_example_no_stdlib_${main_file_name}.cpp.ll"
OUTPUT_LLVM_LINK_1="$output_dir/${main_file_name}_cpp_example_no_stdlib.ll"
OUTPUT_LLVM_LINK_2="$output_dir/${main_file_name}_cpp_example.ll"

# Final outputs:
OUTPUT_CIRCUIT="$output_dir/circuit.crct"
OUTPUT_TABLE="$output_dir/assignment_table.tbl"


# Compile the program to LLVM IR
$clang -DALEN=${alen} -DCLEN=${clen} -DPLEN=${plen} -DMAXLEN=${maxlen} -target assigner \
    -Xclang -fpreserve-vec3-type -Werror=unknown-attributes \
    -D_LIBCPP_ENABLE_CXX17_REMOVED_UNARY_BINARY_FUNCTION -D__ZKLLVM__ \
    -I "${crypto3_lib_dir}"/libs/algebra/include \
    -I "${ZKLLVM_ROOT}"/build/include \
    -I "${crypto3_lib_dir}"/libs/block/include \
    -I /usr/include \
    -I "${ZKLLVM_ROOT}"/libs/blueprint/include \
    -I "${crypto3_lib_dir}"/libs/codec/include \
    -I "${crypto3_lib_dir}"/libs/containers/include \
    -I "${crypto3_lib_dir}"/libs/hash/include \
    -I "${crypto3_lib_dir}"/libs/kdf/include \
    -I "${crypto3_lib_dir}"/libs/mac/include \
    -I "${crypto3_lib_dir}"/libs/marshalling/core/include \
    -I "${crypto3_lib_dir}"/libs/marshalling/algebra/include \
    -I "${crypto3_lib_dir}"/libs/marshalling/multiprecision/include \
    -I "${crypto3_lib_dir}"/libs/marshalling/zk/include \
    -I "${crypto3_lib_dir}"/libs/math/include \
    -I "${crypto3_lib_dir}"/libs/modes/include \
    -I "${crypto3_lib_dir}"/libs/multiprecision/include \
    -I "${crypto3_lib_dir}"/libs/passhash/include \
    -I "${crypto3_lib_dir}"/libs/pbkdf/include \
    -I "${crypto3_lib_dir}"/libs/threshold/include \
    -I "${crypto3_lib_dir}"/libs/pkpad/include \
    -I "${crypto3_lib_dir}"/libs/pubkey/include \
    -I "${crypto3_lib_dir}"/libs/random/include \
    -I "${crypto3_lib_dir}"/libs/stream/include \
    -I "${crypto3_lib_dir}"/libs/vdf/include \
    -I "${crypto3_lib_dir}"/libs/zk/include \
    -I "${ZKLLVM_ROOT}"/libs/stdlib/libcpp \
    -I "${ZKLLVM_ROOT}"/libs/circifier/clang/lib/Headers \
    -I "${ZKLLVM_ROOT}"/libs/stdlib/libc/include \
    -I "${main_file_path}/../../cpp/src" \
    -I "${main_file_path}/../../cpp" \
    -emit-llvm -O1 -S -std=c++20 "$main_source" -o "$OUTPUT_CLANG"

if [ $? -ne 0 ]; then
    echo "Error: The program could not be compiled to LLVM IR"
    exit 1
fi

# Link the program with the ZKLLVM libc
${llvm_link} -S "$OUTPUT_CLANG" -o "$OUTPUT_LLVM_LINK_1"
${llvm_link} -S "$OUTPUT_LLVM_LINK_1" "${LIB_C}/zkllvm-libc.ll" -o "$OUTPUT_LLVM_LINK_2"

if [ $? -ne 0 ]; then
    echo "Error: The program could not be linked with the ZKLLVM libc"
    exit 1
fi

# Check if the assigner is available
assigner_is_available "/dev/stderr" || exit 0

# If stasts is not set then no output will be printed
if [ -z "$stats" ]; then
    # Generate the circuit and the assignment table
    if [ "$verbose" ]; then
        assigner -b "$OUTPUT_LLVM_LINK_2" -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -e pallas
    else
        assigner -b "$OUTPUT_LLVM_LINK_2" -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -e pallas > /dev/null
    fi

    if [ $? -ne 0 ]; then
        echo "Error: The circuit could not be generated"
        exit 1
    fi

    if [ $transpiler == true ]; then
        # Check if the transpiler is available
        transpiler_is_available "/dev/stderr" || exit 0

        # Generate the test proof
        if [ "$verbose" ]; then
            transpiler -m gen-test-proof -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -e pallas -o "$output_dir"
        else
            transpiler -m gen-test-proof -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -e pallas -o "$output_dir" > /dev/null
        fi

        if [ $? -ne 0 ]; then
            echo "Error: The proof could not be generated"
            exit 1
        fi
    else
        if [ $proof_producer == true ]; then
            # Check if the proof-producer is available
            proof_producer_is_available "/dev/stderr" || exit 0

            # Generate the test proof
            if [ "$verbose" ]; then
                $PROOF_PRODUCER_BIN -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE"
            else
                $PROOF_PRODUCER_BIN -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" > /dev/null
            fi
        fi
    fi
    exit 0
fi

# If $log_file is not set, then we will print the output to stdout
if [ -z "$log_file" ]; then
    log_file="/dev/stdout"
fi


# Generate the circuit and the assignment table
[ "$verbose" ] && echo "Generate circuit" >> "$log_file"
TIME1=$(date +%s%3N);

if [ "$verbose" ]; then
    assigner -b "$OUTPUT_LLVM_LINK_2" -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -e pallas
else
    assigner -b "$OUTPUT_LLVM_LINK_2" -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -e pallas > /dev/null
fi

TIME2=$(date +%s%3N);
CIRCUIT_TIME=$(("$TIME2" - "$TIME1"))
echo "$input_filename circuit generation .." $(("$CIRCUIT_TIME" / 1000)).$(("$CIRCUIT_TIME" % 1000)) "s" >> "$log_file"

if [ $transpiler == true ]; then
    # Check if the transpiler is available
    transpiler_is_available "/dev/stderr" || exit 0

    # Generate the test proof
    [ "$verbose" ] && echo "Generate proof and verify with transpiler" >> "$log_file"
    TIME3=$(date +%s%3N);

    if [ "$verbose" ]; then
        transpiler -m gen-test-proof -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -o "$output_dir"
    else
        transpiler -m gen-test-proof -i "$tmp_input_file" -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" -o "$output_dir" > /dev/null
    fi

    TIME4=$(date +%s%3N);
    PROOF_TIME=$(("$TIME4" - "$TIME3"))

    echo "$input_filename proof+verification .." $(("$PROOF_TIME" / 1000)).$(("$PROOF_TIME" % 1000)) "s" >> "$log_file"
    exit 0
else
    if [ $proof_producer == true ]; then
        # Check if the proof-producer is available
        proof_producer_is_available "/dev/stderr" || exit 0

        # Generate the test proof
        echo "Generate proof and verify with proof-producer using $PROOF_PRODUCER_MODE mode" >> "$log_file"
        echo "Warning: The proof-producer doesn't support the -o/--output flag. The output will be saved in the current directory." >> "$log_file"
        TIME3=$(date +%s%3N);

        if [ "$verbose" ]; then
            $PROOF_PRODUCER_BIN -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE"
        else
            $PROOF_PRODUCER_BIN -c "$OUTPUT_CIRCUIT" -t "$OUTPUT_TABLE" > /dev/null
        fi

        TIME4=$(date +%s%3N);
        PROOF_TIME=$(("$TIME4" - "$TIME3"))

        echo "$input_filename proof+verification .." $(("$PROOF_TIME" / 1000)).$(("$PROOF_TIME" % 1000)) "s" >> "$log_file"
        exit 0
    fi
fi
