#!/bin/bash

if [ ${#@} == 0 ]; then
	options=(--release)
else
	options=()
fi

rm -fR static/.gitignore static/*
wasm-pack build --no-typescript --target web --out-name wasm --out-dir ./static "${options[@]}" "$@"
rc=$?
rm -fR static/{.gitignore,package.json}

exit $rc
