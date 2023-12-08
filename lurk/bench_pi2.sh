#!/bin/bash
# Usage ./bench_pi2.sh <output_file>

declare -a arr=("test_transfer_task_specific.lurk"          \
                "test_impreflex_compressed_goal.lurk"       \
                "test_svm5_goal.lurk"                       \
                "test_transfer_simple_compressed_goal.lurk" \
                "test_perceptron_goal.lurk"                 \
                "test_transfer_batch_1k_goal.lurk"          
                )

for f in "${arr[@]}";
do
    echo "$f";
    START_PROVE_TIME=$(date +%s%3N);
    prove_output=$(lurk --rc 400 "${f}")
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
