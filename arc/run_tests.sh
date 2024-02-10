#!/usr/bin/bash

for file in $(ls ./testcases);
do
    echo "Running test $file"
    ./target/debug/arc ./testcases/$file
done
