#!/bin/sh

if [[ "$OSTYPE" == "darwin"* ]]; then
	# if current OS is `darwin` then avoid running `cargo watch` by default
	# refer to: https://github.com/passcod/cargo-watch/issues/129
	./build.sh --dev -- --features dev && simple-http-server -i --nocache --cors
else
	exec cargo watch -s './build.sh --dev -- --features dev && simple-http-server -i --nocache --cors' -w src
fi
