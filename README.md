# Path of Perdition
Welcome to Path of Perdition. A cross platform RPG game written in rust, inspired by classic action RPG's.

## Development

### Compile
#### Desktop
```
cargo run
cargo run --release
```

#### Android

**Run**
```
cargo apk run -p path-to-perdition-android
```

#### WebAssembly

**Run**
```
WASM_SERVER_RUNNER_DIRECTORY="./crates/game" cargo run --target wasm32-unknown-unknown
```
