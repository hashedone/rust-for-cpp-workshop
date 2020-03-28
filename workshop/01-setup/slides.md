# Environment setup

---

## Rust up
### Buildsystem manager

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

<https://www.rust-lang.org/learn/get-started>

---

## Cargo buildsystem

`cargo new` for creating new project

`cargo build` for building project

`cargo run` for running binary

`cargo --help` for help

---

## Consistent formatting

`rustup component add rustfmt`

`cargo fmt` for code formatting

`cargo fmt --check`for formatting check

---

## Extra compile time lints

`rustup component add clippy`

`cargo clippy` for linter pass

---

## Language Server Protocol

### RLS - slow, but stable

`rustup component add rls rust-analysis rust-src`

### Rust analyzer - new generation, but beta

Installed via plugin manager of your IDE

---

### Other cargo utils

<https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands>

---

### Rust playground

<https://play.rust-lang.org/>

---

## Hello World!

```rust
fn main() {
    println!("Hello world!");
}
```

