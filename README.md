# Wasm `std::env::current_dir` repro

```
cargo build
wasmtime run --dir=$PWD --env PWD ./target/wasm32-wasip1/debug/wasm-current-dir-repro.wasm
```

## Output

### macOS

```
PWD: "/Users/runner/work/wasmtime-current-dir-repro/wasmtime-current-dir-repro"
Before: "/"
After: "/Users/runner/work/wasmtime-current-dir-repro/wasmtime-current-dir-repro"
```

### Linux

```
PWD: "/home/runner/work/wasmtime-current-dir-repro/wasmtime-current-dir-repro"
Before: "/"
After: "/home/runner/work/wasmtime-current-dir-repro/wasmtime-current-dir-repro"
```

### Windows

```
PWD: "D:/a/wasmtime-current-dir-repro/wasmtime-current-dir-repro"
Before: "/"
After: "/D:/a/wasmtime-current-dir-repro/wasmtime-current-dir-repro"
```
