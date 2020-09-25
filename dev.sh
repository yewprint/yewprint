#!/bin/sh

exec cargo watch -s './build.sh --dev -- --features doc && simple-http-server -i --nocache --cors public/' -i /public
