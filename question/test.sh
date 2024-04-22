#!/bin/bash


# Loop through all JavaScript files
for file in *.js; do
    if [ -f "$file" ]; then
        echo "Running $file..."
        node "$file"
        echo "Finished $file."
        echo ""
    fi
done
