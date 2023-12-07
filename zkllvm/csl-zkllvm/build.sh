#!/bin/bash

if [[ "$#" -lt 2 ]]; then
  echo "Usage ./build <program> <input> [-v/--verbose]"
  exit 1
fi

# Check if ZKLLVM_ROOT is set
if [[ -z "${ZKLLVM_ROOT}" ]]; then
  echo "ZKLLVM_ROOT is not set"
  exit 1
fi

if [[ "$#" -eq 3 ]]; then
  if [[ "${3}" == "-v" || "${3}" == "--verbose" ]]; then
    set -x
  fi
fi

FILE=$(basename "${1}")
FILEPATH=$(dirname "$(realpath "${1}")")
FILENAME=${FILE%%.*}
ROOT_DIR=$(dirname "$(realpath "${0}")")/..
OUTPUTDIR="${ROOT_DIR}/.build/output/${FILENAME}"
EXT=${FILE##*.}
INPUT=$2

CRYPTO3_LIB_DIR="${ZKLLVM_ROOT}"/libs/crypto3
CLANG_EXE="${ZKLLVM_ROOT}"/build/libs/circifier/llvm/bin/clang-16
LLVM_LINK="${ZKLLVM_ROOT}"/build/libs/circifier/llvm/bin/llvm-link
LIB_C="${ZKLLVM_ROOT}/build/libs/stdlib/libc"
ASSIGNER="${ZKLLVM_ROOT}"/build/bin/assigner/assigner
TRANSPILER="${ZKLLVM_ROOT}"/build/bin/transpiler/transpiler

# Intermediary outputs:
OUTPUT_CLANG="${OUTPUTDIR}/${FILENAME}_${EXT}_example_no_stdlib_${FILENAME}.${EXT}.ll"
OUTPUT_LLVM_LINK_1="${OUTPUTDIR}/${FILENAME}_${EXT}_example_no_stdlib.ll"
OUTPUT_LLVM_LINK_2="${OUTPUTDIR}/${FILENAME}_${EXT}_example.ll"

# Final outputs:
OUTPUT_CIRCUIT="${OUTPUTDIR}/circuit.crct"
OUTPUT_TABLE="${OUTPUTDIR}/assignment_table.tbl"

# Creating output directory under `.build` if it doesn't exist or clean it if it does
if [[ ! -d "${ROOT_DIR}/.build/output" ]]; then
  mkdir -p "${ROOT_DIR}/.build/output"
fi
if [[ ! -d "${OUTPUTDIR}" ]]; then
  mkdir "${OUTPUTDIR}"
else
  rm -r "${OUTPUTDIR:?}/"
  mkdir "${OUTPUTDIR}"
fi

# Compile the program to LLVM IR
${CLANG_EXE} -target assigner -D__ZKLLVM__ \
-I "${CRYPTO3_LIB_DIR}"/libs/algebra/include \
-I "${ZKLLVM_ROOT}"/build/include \
-I "${CRYPTO3_LIB_DIR}"/libs/block/include \
-I  /usr/include \
-I "${ZKLLVM_ROOT}"/libs/blueprint/include \
-I "${CRYPTO3_LIB_DIR}"/libs/codec/include \
-I "${CRYPTO3_LIB_DIR}"/libs/containers/include \
-I "${CRYPTO3_LIB_DIR}"/libs/hash/include \
-I "${CRYPTO3_LIB_DIR}"/libs/kdf/include \
-I "${CRYPTO3_LIB_DIR}"/libs/mac/include \
-I "${CRYPTO3_LIB_DIR}"/libs/marshalling/core/include \
-I "${CRYPTO3_LIB_DIR}"/libs/marshalling/algebra/include \
-I "${CRYPTO3_LIB_DIR}"/libs/marshalling/multiprecision/include \
-I "${CRYPTO3_LIB_DIR}"/libs/marshalling/zk/include \
-I "${CRYPTO3_LIB_DIR}"/libs/math/include \
-I "${CRYPTO3_LIB_DIR}"/libs/modes/include \
-I "${CRYPTO3_LIB_DIR}"/libs/multiprecision/include \
-I "${CRYPTO3_LIB_DIR}"/libs/passhash/include \
-I "${CRYPTO3_LIB_DIR}"/libs/pbkdf/include \
-I "${CRYPTO3_LIB_DIR}"/libs/threshold/include \
-I "${CRYPTO3_LIB_DIR}"/libs/pkpad/include \
-I "${CRYPTO3_LIB_DIR}"/libs/pubkey/include \
-I "${CRYPTO3_LIB_DIR}"/libs/random/include \
-I "${CRYPTO3_LIB_DIR}"/libs/stream/include \
-I "${CRYPTO3_LIB_DIR}"/libs/vdf/include \
-I "${CRYPTO3_LIB_DIR}"/libs/zk/include \
-I "${ZKLLVM_ROOT}"/libs/stdlib/libcpp \
-I "${ZKLLVM_ROOT}"/libs/stdlib/libc/include \
-I "${FILEPATH}/../../cpp/src" \
-emit-llvm -O1 -S -std=c++20 "${FILEPATH}/${FILE}" -o "${OUTPUT_CLANG}"

# Link the program with the ZKLLVM libc
${LLVM_LINK} -S "${OUTPUT_CLANG}" -o "${OUTPUT_LLVM_LINK_1}"
${LLVM_LINK} -S "${OUTPUT_LLVM_LINK_1}" "${LIB_C}/zkllvm-libc.ll" -o "${OUTPUT_LLVM_LINK_2}"

# # Generate the circuit and the assignment table
echo "Generate circuit"
TIME1=$(date +%s%3N);

${ASSIGNER} -b "${OUTPUT_LLVM_LINK_2}" -i "${INPUT}" -c "${OUTPUT_CIRCUIT}" -t "${OUTPUT_TABLE}" -e pallas --print_circuit_output

TIME2=$(date +%s%3N);
CIRCUIT_TIME=$(expr $TIME2 - $TIME1)
echo $FILENAME "circuit generation .." $(expr $CIRCUIT_TIME / 1000).$(expr $CIRCUIT_TIME % 1000) "s"

# Generate the test proof
echo "Generate proof and verify"
TIME3=$(date +%s%3N);

${TRANSPILER} -m gen-test-proof -i "${INPUT}" -c "${OUTPUT_CIRCUIT}" -t "${OUTPUT_TABLE}" -o "${OUTPUTDIR}"

TIME4=$(date +%s%3N);
PROOF_TIME=$(expr $TIME4 - $TIME3)

echo $FILENAME "proof+verification .." $(expr $PROOF_TIME / 1000).$(expr $PROOF_TIME % 1000) "s"
