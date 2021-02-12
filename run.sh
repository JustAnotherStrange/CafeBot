#!/bin/sh

cargo build --release
if [ $? != 0 ]; then
    exit
fi
# auto restart
while true;
	do date
    	./target/release/CafeBot
done

