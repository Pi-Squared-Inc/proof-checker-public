#!/bin/bash
# Usage: ./bench.sh <output_file>

declare -a arr=("perceptron.cairo"     \
                "svm5.cairo"           \
                "transfer.cairo"       \
                "transfer_batch.cairo" \
               )
BUILDDIR=./.build

if [[ ! -d "${BUILDDIR}" ]]; then
  mkdir "${BUILDDIR}"
else
  rm -r "${BUILDDIR:?}/"
  mkdir "${BUILDDIR}"
fi

PWD=$(pwd);

for f in "${arr[@]}"; do
    echo "$f";
    filename=${f%%.*}
    mkdir "${BUILDDIR}/${filename}"
    echo "$f" >> "${PWD}/$1"

    # Compile
    cairo-compile ${filename}.cairo --output ${BUILDDIR}/${filename}/${filename}.json --proof_mode

    # Execute
    START=$(date +%s%3N);
    cairo-run --program=${BUILDDIR}/${filename}/${filename}.json --layout=small --print_output --print_info --proof_mode --trace_file=${BUILDDIR}/${filename}/${filename}_trace.json --memory_file=${BUILDDIR}/${filename}/${filename}_memory.json > /dev/null
    END=$(date +%s%3N);
    TOTAL=$(("$END" - "$START"))
    echo "Execution time" ".." $(("$TOTAL" / 1000)).$(("$TOTAL" % 1000)) "s" >> "$PWD/$1"

    # Prove
    echo "Prove .."  >> "$PWD/$1"
    platinum-prover prove ${BUILDDIR}/${filename}/${filename}_trace.json ${BUILDDIR}/${filename}/${filename}_memory.json ${BUILDDIR}/${filename}/${filename}.proof | grep -E "Time spent in proving:" >> "$PWD/$1"

    # Verify
    echo "Verify .." >> "$PWD/$1"
    platinum-prover verify ${BUILDDIR}/${filename}/${filename}.proof | grep -E "Time spent in verifying:" >> "$PWD/$1"

    echo -e "\n" >> "$PWD/$1";
done;
