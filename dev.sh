#!/bin/sh

exec cargo watch -s './build.sh --dev && simple-http-server -i --nocache --cors' -w src
