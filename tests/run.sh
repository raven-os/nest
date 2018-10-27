#!/usr/bin/env bash

# Common variables shared among all tests
export NEST="cargo run --bin=nest"

# Colors
export RED="\033[1;31m"
export GREEN="\033[1;32m"
export RESET="\033[0m"

declare -i nb_tests=1
declare -i success=0
declare tests_dir=$(dirname "$0")

# Run all tests
for ((i=1; i <= $nb_tests; i++)) do
	$tests_dir/test_$i/run.sh > /dev/null 2> /dev/null
	declare -i out_code=$?

	if [[ $out_code -eq 0 ]]; then
		printf "[%02i] ${GREEN}OK${RESET}\n" $i
		success=$(($success + 1))
	else
		printf "[%02i] ${RED}KO${RESET}\n" $i
	fi
done

echo
echo "$success/$nb_tests tests passed"

# Exit 1 if any test failed to ensure the build fails on Travis
if [[ $success -ne $nb_tests ]]; then
	exit 1
fi
