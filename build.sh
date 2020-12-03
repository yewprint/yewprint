#!/bin/bash

if [ ${#@} == 0 ]; then
	options=(--release)
else
	options=()
fi

if ! [ -f core.tgz ]; then
	curl -o core.tgz https://registry.npmjs.org/@blueprintjs/core/-/core-3.36.0.tgz
fi

if ! [ -f docs-theme.tgz ]; then
	curl -o docs-theme.tgz https://registry.npmjs.org/@blueprintjs/docs-theme/-/docs-theme-3.7.1.tgz
fi

# cleanup
mkdir -p public
rm -fR public/.gitignore public/*

# copy index.html
cp static/* public/

# copy favicon
cp yewprint-doc/src/logo.svg public/favicon.svg

# download blueprint css
tar xzOf core.tgz package/lib/css/blueprint.css > public/blueprint.css

# download blueprint doc css
tar xzOf docs-theme.tgz package/lib/css/docs-theme.css > public/docs-theme.css

# build
(cd yewprint-doc && \
	wasm-pack build --no-typescript --target web --out-name wasm \
		--out-dir ../public "${options[@]}" "$@")
rc=$?

rm -fR public/{.gitignore,package.json,README.md}

echo Wasm size: $(cat public/*.wasm | wc -c)

exit $rc
