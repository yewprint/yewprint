#!/bin/bash

if [ ${#@} == 0 ]; then
	options=(--release -- --features dev)
else
	options=()
fi

if ! [ -f core.tgz ]; then
	curl -o core.tgz https://registry.npmjs.org/@blueprintjs/core/-/core-3.30.0.tgz
fi

# cleanup
mkdir -p public
rm -fR public/.gitignore public/*

# copy index.html
cp static/index.html public/

# download blueprint css
tar xzOf core.tgz package/lib/css/blueprint.css > public/blueprint.css

# build
wasm-pack build --no-typescript --target web --out-name wasm --out-dir ./public "${options[@]}" "$@"
rc=$?

rm -fR public/{.gitignore,package.json,README.md}

exit $rc
