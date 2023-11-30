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
START=$(date +%s%3N);
# To see the iterations, comment "> /dev/null"
lurk --rc 400 "${f}" > /dev/null;
END=$(date +%s%3N);
TOTAL=$(("$END" - "$START"))
echo "$f" ".." $(("$TOTAL" / 1000)).$(("$TOTAL" % 1000)) "s" >> "$1";
done;
