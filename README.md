# class-warfare

[![crates.io](https://img.shields.io/crates/v/class-warfare.svg)](https://crates.io/crates/class-warfare)
[![Documentation](https://docs.rs/class-warfare/badge.svg)](https://docs.rs/class-warfare)
[![CI](https://github.com/mirged/class-warfare/actions/workflows/ci.yml/badge.svg)](https://github.com/mirged/class-warfare/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

**Composition wearing an inheritance-shaped trench coat.**

A dependency-free, `no_std` Rust library with a `class!` macro for class syntax,
constructors, and parent delegation through safe composition.

A Rust class experiment that escaped `main.rs`, formed a union, and became a library.

The reusable crate lives in [crates/class-warfare](crates/class-warfare/README.md).
It includes the original `class!` syntax, parent delegation, custom derives,
`no_std` support, examples, and tests. The root binary still demonstrates the
Animal / Dog / GuardDog family tree using that library.

## Installation

```sh
cargo add class-warfare
```

Or add this to `Cargo.toml` (Rust 1.85 or newer):

```toml
[dependencies]
class-warfare = "0.1.0"
```

## Quick start

```rust
use class_warfare::class;

class! { class Employee { pub meetings: u32 } }
class! { class Manager extends Employee { pub reports: u32 } }

let mut boss = Manager::new(Employee::new(3), 7);
boss.meetings += 1;
assert_eq!(boss.super_cast().meetings, 4);
```

Derived classes embed a parent and delegate through `Deref` and `DerefMut`.
Method shadowing is static; use Rust traits when you need runtime dispatch.
See the [full guide](crates/class-warfare/README.md) for syntax and limitations,
or browse the [API documentation](https://docs.rs/class-warfare).

## Examples and development

```text
cargo run
cargo run -p class-warfare --example office_politics
cargo run -p class-warfare --example family_business
cargo test --workspace
```

Bug reports and contributions are welcome in
[GitHub issues](https://github.com/mirged/class-warfare/issues) and pull requests.
Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
and `cargo test --workspace` before submitting a change.

Licensed under [MIT](LICENSE-MIT). The inheritance tax is zero.
