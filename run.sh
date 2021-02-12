#!/bin/sh

cargo build --release
if [ $? != 0 ]; then
    exit
fi
# auto restart
while true;
    do ./target/release/CafeBot
done

