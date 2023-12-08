#!/bin/bash
# Usage: ./bench.sh <output_file>

PWD=$(pwd);
for d in */; do
echo "$d";
cd "$d" || exit;
scarb build > /dev/null;
START=$(date +%s%3N);
scarb cairo-run --available-gas 1000000000000000000 > /dev/null;
END=$(date +%s%3N);
cd .. || exit;
TOTAL=$(("$END" - "$START"))
echo "$d" ".." $(("$TOTAL" / 1000)).$(("$TOTAL" % 1000)) "s" >> "$PWD/$1";
done;
