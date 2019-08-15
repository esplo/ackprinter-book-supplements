#!/usr/bin/env bash

set -eu

for i in `seq 0 $1`; do
    for j in `seq 0 $2`; do
        echo -n "${i},${j},"
        gtime -f '%e,%M' ./target/debug/print_naive $i $j > /dev/null
    done
done
