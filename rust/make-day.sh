#!/usr/bin/env bash

if [ $# != 1 ]; then
	echo "Usage: $(basename "$0") <day-number>" >&2
	exit 1
fi
# if [ ! -d .git ]; then
#     echo "must be run from root of advent-of-code repository" >&2
#     exit 1
# fi

name=$(echo "aoc$1")
cargo new --bin "$name"
mkdir "$name/input"
touch "$name/input/sample.txt"
touch "$name/input/input.txt"
