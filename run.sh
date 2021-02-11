#!/bin/sh

cargo build
# auto restart
while true;
    do ./target/debug/CafeBot
done

