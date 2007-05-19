#find . -name "*.wasm" -type f -delete

for f in $(find . -name "*.wasm"); do
    if [[ $f =~ "output" ]]; then
        wasm-opt -Oz $f -o tmp.wasm
        mv tmp.wasm $f
    fi
done
