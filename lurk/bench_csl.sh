#!/bin/bash
# Usage ./bench_csl.sh <output_file>

cd csl-examples || exit;

for f in *;
do
echo "$f";
START=$(date +%s%3N);
if [ "$f" == "transfer5000.lurk" ]; then
# To see the iterations, comment "> /dev/null"
lurk --rc 400 "${f}"  > /dev/null;
else
lurk "${f}"  > /dev/null;
fi
END=$(date +%s%3N);
TOTAL=$(("$END" - "$START"))
echo "$f" ".." $(("$TOTAL" / 1000)).$(("$TOTAL" % 1000)) "s" >> "$1";
done;
mv "$1" ..
