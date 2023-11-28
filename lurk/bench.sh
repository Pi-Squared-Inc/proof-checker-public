#!/bin/bash
cd csl-examples || exit;

for f in *;
do
echo "$f";
START=$(date +%s%3N);
lurk "${f}" > /dev/null;
END=$(date +%s%3N);
TOTAL=$(("$END" - "$START"))
echo "$f" ".." $(("$TOTAL" / 1000)).$(("$TOTAL" % 1000)) "s" >> "$1";
done;
mv "$1" ..
