#!/bin/bash

if [ ${#@} == 0 ]; then
	options=(--release)
else
	options=()
fi

if ! [ -f core.tgz ]; then
	curl -o core.tgz https://registry.npmjs.org/@blueprintjs/core/-/core-3.30.0.tgz
fi

mkdir -p static

rm -fR static/.gitignore static/*

# decompress the tar file but extract the `.css` files only
# find every file in the nested directories and move it behind `./static`
# finally remove every directory behind `./static`
bsdtar xvzf core.tgz -C static \*.css &&
	find ./static -mindepth 2 -type f -print -exec mv {} ./static \; &&
	find ./static -mindepth 1 -type d -print -exec rm -rf {} \;

wasm-pack build --no-typescript --target web --out-name wasm --out-dir ./static "${options[@]}" "$@"

rc=$?

rm -fR static/{.gitignore,package.json}

exit $rc
