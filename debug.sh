#! /bin/sh

make build > /dev/null
./target/debug/vt "$@"
