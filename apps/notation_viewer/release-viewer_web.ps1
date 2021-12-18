git checkout release-wasm
git merge main

cargo make --profile release release-web
