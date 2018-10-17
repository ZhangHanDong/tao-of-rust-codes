# 安装wasm-gc

```
cargo install wasm-gc
```

# 生成wasm的三条命令：

```
$ cargo build --target wasm32-unknown-unknown 
$ cp target/wasm32-unknown-unknown/debug/hello_wasm.wasm output
$ wasm-gc output/hello_wasm.wasm output/small_hello.wasm
```