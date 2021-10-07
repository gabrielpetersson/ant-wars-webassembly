wasm:
  wasm-pack build

build:
  cargo build && just wasm
