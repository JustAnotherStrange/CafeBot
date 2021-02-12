#!/bin/sh

cargo build --release
# auto restart
while true;
    do ./target/release/CafeBot
done

