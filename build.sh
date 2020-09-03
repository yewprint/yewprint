#!/bin/bash

if [ ${#@} == 0 ]; then
	options=(--release)
else
	options=()
fi

if ! [ -f core.tgz ]; then
	curl -o core.tgz https://registry.npmjs.org/@blueprintjs/core/-/core-3.30.0.tgz
fi

rm -fR static/.gitignore static/*
tar xvzf core.tgz -C static --wildcards \*.css --transform='s/.*\///'
wasm-pack build --no-typescript --target web --out-name wasm --out-dir ./static "${options[@]}" "$@"
rc=$?
rm -fR static/{.gitignore,package.json}

exit $rc
