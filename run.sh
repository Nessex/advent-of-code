#!/bin/bash

part=${1:?provide a part as the first argument e.g. 2020d1p1}

cat "2020/aoc-${part}/input.txt" | cargo run --bin "aoc-${part}"
