## Trying out [macroquad](https://github.com/not-fl3/macroquad)

### Run the sample

```sh
cargo run --bin moving_square
```

### Build it

```sh
cargo build --release --bin square
```

### Build for windows 

```sh
rustup target add x86_64-pc-windows-msvc
cargo build --target x86_64-pc-windows-msvc --release --bin square
```

### Build for wasm 

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release --bin square
```

    