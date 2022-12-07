#!/bin/sh

cargo b && sudo RUST_BACKTRACE=1 ./target/debug/yclass
