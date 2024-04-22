#!/usr/bin/bash

for file in $(ls ./testcases);
do
    echo "Running test $file"
    ./target/debug/arcc ./testcases/$file
done
