#!/usr/bin/env bash

# Common variables shared among all tests
export CARGO="env cargo"
export NEST_SERVER="/tmp/nest-server"
export PYTHONPATH=$(dirname "$0")
export NEST_CHROOT="/tmp/chroot/"

# Colors
export RED="\033[1;31m"
export GREEN="\033[1;32m"
export RESET="\033[0m"

if [ ! -d $NEST_SERVER ]; then
    echo "Cloning latest nest-server..."
    git clone https://github.com/raven-os/nest-server $NEST_SERVER
elif [ ! -e $NEST_SERVER ]; then
    echo "$NEST_SERVER already exists and is not a directory, aborting."
    exit 1
fi

pushd $NEST_SERVER
$CARGO build
popd

if [ -e $NEST_CHROOT ]; then
    echo "$NEST_CHROOT already exists, aborting."
    exit 1
fi

declare -i nb_tests=0
declare -i success=0
declare tests_dir=$(dirname "$0")

# Run all tests
for test in $tests_dir/test_*; do
    $test/run.py
    declare -i out_code=$?

    if [[ $out_code -eq 0 ]]; then
	printf "[%02i] ${GREEN}OK${RESET}\n" $nb_tests
	success=$(($success + 1))
    else
	printf "[%02i] ${RED}KO${RESET}\n" $nb_tests
    fi
    nb_tests=$(($nb_tests + 1))
    sudo rm -rf $NEST_CHROOT
done

echo
echo "$success/$nb_tests tests passed"

# Exit 1 if any test failed to ensure the build fails on Travis
if [[ $success -ne $nb_tests ]]; then
    exit 1
fi
