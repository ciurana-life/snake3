# Snake 3

Play the classical snake game on your terminal or use the crate to build your own.

![snake game](https://raw.githubusercontent.com/ciurana-life/snake3/refs/heads/main/img/image.png)

## Install
Either with:
```bash
cargo add snake3@=0.1.1
```

Or add to Cargo.toml:
```toml
snake3 = "=0.1.1"
```

## Play
```bash
cargo run
```

## You can use it for WASM
Set this flag:
```bash
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
```
Compile:
```bash
cargo build --release --target wasm32-unknown-unknown
```
Remember to remove flags after if you want to use `main.rs` game:
```bash
unset RUSTFLAGS
```


## Docs

[https://docs.rs/snake3/0.1.0/snake3/](https://docs.rs/snake3/0.1.0/snake3/)
