#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 [--full] '<command_to_run_program>'"
    echo "Example: $0 --full 'python main.py'"
}

# Check if a program is provided
if [[ $# -eq 1 && ("$1" == "--help" || "$1" == "-h") ]]; then
    usage
    exit 1
fi


if [ $# -eq 2 ] && [ "$1" == "--full" ]; then
    TEST_FULL=true
    shift
else
    TEST_FULL=false
fi

if [[ $# -eq 0 || ($# -eq 1 && "$1" == "--full")]]; then
    printf "Error: No command provided.\n\n"
    usage
    exit 1
fi

# Program or command to run (e.g., "./program")
PROGRAM="$1"

# Define test names, test cases, and expected outputs
declare -a TEST_NAMES=(
    "closed_stdin"
    "piped_input"
    "empty"
    "empty_no_newline"
    "only_header"
    "random"
    "missing_ean_column"
    "too_many_fields"
    "too_few_fields"
    "all_valid"
    "with_gtin_8_and_12"
    "empty_lines"
    "leading_zeros"
    "ean_column_moved"
    "emoji"
    "missing_ean"
    "too_long_ean"
    "too_short_ean"
    "with_garbage"
    "wrong_checksum"
    "quoted_fields"
    "misquoted_fields"
    "mixed_5k"
)

declare -a TEST_CASES=(
    "$PROGRAM <&-"
    "echo 'ean' | $PROGRAM"
    "echo '' | $PROGRAM"
    "echo -n '' | $PROGRAM"
    "echo 'ean,price,quantity,brand,color' | $PROGRAM"
    "cat /dev/urandom | head -c 1000 | $PROGRAM"
    "cat tests/missing_ean_column.csv | $PROGRAM"
    "cat tests/too_many_fields.csv | $PROGRAM"
    "cat tests/too_few_fields.csv | $PROGRAM"
    "cat tests/all_valid.csv | $PROGRAM"
    "cat tests/with_gtin_8_and_12.csv | $PROGRAM"
    "cat tests/empty_lines.csv | $PROGRAM"
    "cat tests/leading_zeros.csv | $PROGRAM"
    "cat tests/ean_column_moved.csv | $PROGRAM"
    "cat tests/emoji.csv | $PROGRAM"
    "cat tests/missing_ean.csv | $PROGRAM"
    "cat tests/too_long_ean.csv | $PROGRAM"
    "cat tests/too_short_ean.csv | $PROGRAM"
    "cat tests/with_garbage.csv | $PROGRAM"
    "cat tests/wrong_checksum.csv | $PROGRAM"
    "cat tests/quoted_fields.csv | $PROGRAM"
    "cat tests/misquoted_fields.csv | $PROGRAM"
    "curl https://stockly-public-assets.s3.eu-west-1.amazonaws.com/peer-programming-mixed.csv | $PROGRAM"
)

declare -a EXPECTED_OUTPUTS=(
    "0 0"
    "0 0"
    "0 0"
    "0 0"
    "0 0"
    "0 0"
    "0 11"
    "10 0"
    "10 0"
    "10 0"
    "10 0"
    "10 0"
    "10 0"
    "10 0"
    "10 0"
    "9 1"
    "9 1"
    "9 1"
    "6 4"
    "8 2"
    "10 0"
    "5 1"
    "4976 17"
)

if [ "$TEST_FULL" = true ]; then
    TEST_NAMES+=("17GiB")
    TEST_CASES+=("curl https://stockly-public-assets.s3.eu-west-1.amazonaws.com/peer-programming-big.csv | $PROGRAM")
    EXPECTED_OUTPUTS+=("185794877 950163")
fi

# Check array lengths
if [ ${#TEST_NAMES[@]} -ne ${#TEST_CASES[@]} ] || [ ${#TEST_CASES[@]} -ne ${#EXPECTED_OUTPUTS[@]} ]; then
    echo "Error: Array lengths do not match"
    exit 1
fi

# Temporary file for storing program output
TMP_OUT=$(mktemp)

NB_TESTS=${#TEST_CASES[@]}
NB_SUCCESS=0
NB_FAILURE=0
# Run each test case
echo "Running tests on program: $PROGRAM"
for i in "${!TEST_CASES[@]}"; do
    TEST_NAME="${TEST_NAMES[$i]}"
    TEST_CMD="${TEST_CASES[$i]}"
    EXPECTED="${EXPECTED_OUTPUTS[$i]}"
    
    echo "Running test: [$TEST_NAME]"
    # Execute the command in a way that handles multi-word commands
    bash -c "$TEST_CMD" > "$TMP_OUT"
    ACTUAL_OUTPUT=$(cat "$TMP_OUT")

    # Compare actual output with expected output
    if [[ "$ACTUAL_OUTPUT" == "$EXPECTED" ]]; then
        echo "PASSED ✅"
        NB_SUCCESS=$((NB_SUCCESS+1))
    else
        echo "FAILED ❌"
        echo "  Expected: '$EXPECTED'"
        echo "  Got:      '$ACTUAL_OUTPUT'"
        NB_FAILURE=$((NB_FAILURE+1))
    fi
done

# Cleanup
rm "$TMP_OUT"

# Print summary
echo "Summary:"
echo "  Total tests: $NB_TESTS"
echo "  Passed: $NB_SUCCESS"
echo "  Failed: $NB_FAILURE"
