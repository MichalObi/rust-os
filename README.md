# Rust OS

1) Install rustup

```
  curl https://sh.rustup.rs -sSf | sh
```

2) Install rust nightly compiler

```
  rustup override add nightly
```
3) Compile with this command (Linux only)
```
cargo rustc -- -Z pre-link-arg=-nostartfiles
```
