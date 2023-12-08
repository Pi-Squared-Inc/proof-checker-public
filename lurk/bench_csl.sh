#!/bin/bash
# Usage ./bench_csl.sh <output_file>

cd csl-examples || exit;

for f in *; do
    echo "$f"

    START_PROVE_TIME=$(date +%s%3N)
    if [ "$f" == "transfer5000.lurk" ]; then
        prove_output=$(lurk --rc 400 "${f}")
    else
        prove_output=$(lurk "${f}")
    fi
    END_PROVE_TIME=$(date +%s%3N);

    proof_key=$(echo "$prove_output" | grep -E "Proof key:" | sed -E 's/Proof key: \"(.*)\"/\1/')

    START_VERIFY_TIME=$(date +%s%3N);
    verify_output=$(lurk verify "$proof_key")
    END_VERIFY_TIME=$(date +%s%3N);

    TOTAL_PROVE_TIME=$(("$END_PROVE_TIME" - "$START_PROVE_TIME"))
    TOTAL_VERIFY_TIME=$(("$END_VERIFY_TIME" - "$START_VERIFY_TIME"))

    # Printing all the output to a file
    {
        echo "Proving $f in" $(("$TOTAL_PROVE_TIME" / 1000)).$(("$TOTAL_PROVE_TIME" % 1000)) "s";
        echo "Verifying $f" "in" $(("$TOTAL_VERIFY_TIME" / 1000)).$(("$TOTAL_VERIFY_TIME" % 1000)) "s";
        echo ""
        echo "$verify_output";
        echo "---------------------------------------------------------------------------------------------------"
    } >> "$1";
done;
mv "$1" ..
