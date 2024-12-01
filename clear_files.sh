#!/bin/bash

# Define the pattern
pattern="*.txt"

# Find and empty the files matching the pattern
for file in $pattern; do
    > "$file"
    echo "File '$file' has been emptied."
done