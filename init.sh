#!/bin/zsh

echo initialising day "$1"

cd ~/Documents/adventofcode/src/ || exit 1

mkdir day"$1"
cd day"$1" || exit 1
touch mod.rs
touch input
touch test-one
touch test-two
touch answer-one
touch answer-two

cat ../../template > mod.rs

cd ..
echo -e "mod day$1;\n$(cat main.rs)" > main.rs
