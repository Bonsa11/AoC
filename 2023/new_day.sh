#!/bin/bash

# Function to display usage information
function display_help {
    echo "Usage: $0 [OPTIONS] <project_name>"
    echo "Options:"
    echo "  --help    Display this help message"
    exit 1
}

# Check if any arguments are provided
if [ "$#" -eq 0 ]; then
    display_help
fi

# Process command-line options
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --help)
            display_help
            ;;
        *)
            project_name="$1"
            ;;
    esac
    shift
done

# Check if the required project name is provided
if [ -z "$project_name" ]; then
    echo "Error: Missing required project name"
    display_help
fi

# Create a new Rust project using Cargo
cargo new "$project_name"

# Navigate to the project directory
cd "$project_name"

# Run once to build dirs
cargo run

# Copy in Templates
mkdir ./src/bin
cp ../day-test/src/bin/part1.rs ./src/bin/part1.rs
cp ../day-test/src/bin/part1.rs ./src/bin/part2.rs

# create template files
touch input.txt test.txt README.md

# End of the script
