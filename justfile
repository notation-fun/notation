run-viewer:
    cd apps/notation_viewer && cargo run --features native

run-kb:
    cd apps/notation_kb && cargo run --features native

install-wasm-bindgen-cli:
    cargo install --force wasm-bindgen-cli --version=0.2.87

install-basic-http-server:
    cargo install basic-http-server

install-tools-mac:
    brew install binaryen
    brew install md5sha1sum

