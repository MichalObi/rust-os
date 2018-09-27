# Rust OS

1) Install rustup

```
  curl https://sh.rustup.rs -sSf | sh
```

2) Install rust nightly compiler

```
  rustup override add nightly
```
3) Install cargo xbuild to cross-compile build-in libs.
```
cargo install cargo-xbuild / rustup component add rust-src (if needed)
```
4) Add bootimage utility to compile kernel and bootloader and combine them both.
```
cargo install bootimage --version "^0.5.0"
```
5) Compile with this command (Linux only) with specific compiler config as target
```
bootimage build
```
