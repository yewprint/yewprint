#!/bin/sh

exec cargo watch -s './build.sh --dev -- --features dev && simple-http-server -i --nocache --cors public/' -w src -w static
