#!/bin/sh

cargo build
touch log
if [ ! -f ./count ]; then
    echo 0 > ./count
fi
# auto restart
while true;
    do ./target/debug/CafeBot
done

