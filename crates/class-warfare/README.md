# class-warfare

[![crates.io](https://img.shields.io/crates/v/class-warfare.svg)](https://crates.io/crates/class-warfare)
[![Documentation](https://docs.rs/class-warfare/badge.svg)](https://docs.rs/class-warfare)
[![CI](https://github.com/mirged/class-warfare/actions/workflows/ci.yml/badge.svg)](https://github.com/mirged/class-warfare/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/mirged/class-warfare/blob/main/LICENSE-MIT)

**Composition wearing an inheritance-shaped trench coat.**

Miss writing `class Dog extends Animal`? Want your Rust code to be summoned
before the Architecture Committee? Welcome. We provide the syntax. You provide
the plausible deniability.

`class-warfare` is a dependency-free, `no_std` library for inheritance-style
syntax using composition. Its `class!` macro generates ordinary structs, constructors,
and parent delegation using safe Rust. No allocation, runtime registry, or
procedural macro is required by the library.

## Installation

Add it to your project:

```sh
cargo add class-warfare
```

Or declare the dependency in `Cargo.toml`:

```toml
[dependencies]
class-warfare = "0.1.0"
```

Requires Rust 1.85 or newer. There are no dependencies or feature flags.

[API documentation](https://docs.rs/class-warfare) ·
[Source and issues](https://github.com/mirged/class-warfare) ·
[Changelog](https://github.com/mirged/class-warfare/blob/main/crates/class-warfare/CHANGELOG.md)

## Your first questionable family tree

```rust
use class_warfare::{class, HasParent};

class! {
    /// An employee of the barking industry.
    class Animal {
        pub name: String,
        pub age: u32,
    }

    pub fn speak(&self) -> &str { "[contractually obligated animal noise]" }

    pub fn celebrate_birthday(&mut self) { self.age += 1; }
}

class! {
    class Dog extends Animal {
        pub borks: u32,
    }

    pub fn speak(&self) -> &str { "BORK. This meeting could have been a walk." }

    pub fn bark(&mut self) { self.borks += 1; }
}

class! {
    class GuardDog extends Dog {
        pub clearance: u8,
    }
}

let mut guard = GuardDog::new(
    Dog::new(Animal::new("Sir Borks-a-Lot".into(), 4), 0),
    7,
);
guard.bark();                        // Dog's method
guard.celebrate_birthday();          // Animal's method, through two parents
assert_eq!(guard.age, 5);            // Parent field access works too
assert_eq!(guard.borks, 1);
assert_eq!(guard.speak(), "BORK. This meeting could have been a walk.");

fn inspect(animal: &Animal) -> &str { animal.speak() }
assert_eq!(inspect(&guard), "[contractually obligated animal noise]");
assert_eq!(guard.parent().borks, 1);  // HasParent always means DIRECT parent
assert_eq!(guard.super_cast().super_cast().age, 5);
```

## The benefits package

| Generated feature | Base | Derived |
| --- | --- | --- |
| `Debug`, `Clone` by default | Yes | Yes |
| `new(fields...)` | Yes | Parent first, then own fields |
| Public `super_class` field | No | Direct parent |
| `Deref`, `DerefMut` | No | Direct parent |
| `super_cast()`, `super_cast_mut()` | No | Direct parent references |
| `into_super()` | No | Consumes child, returns direct parent |
| `HasParent` | No | `parent`, `parent_mut`, `into_parent` |
| `AsRef<Parent>`, `AsMut<Parent>` | No | Direct parent references |

`class Name` is public for compatibility with the original macro. Use
`pub(crate) class Name`, `pub(super) class Name`, or another explicit Rust
visibility to restrict it. Fields keep their declared visibility; omitted
field visibility is private to the containing module, just like a normal struct.
The generated constructor and parent helpers are public.

After the field block, write ordinary inherent implementation items: public or
private methods, associated constants, factories, generic methods, `async fn`,
and `where` clauses. Traits are implemented separately using normal Rust.
One macro invocation declares one class.

```rust
use class_warfare::class;

class! {
    #[derive(PartialEq, Eq)]
    pub(crate) class Meeting {
        /// Minutes elapsed; productivity is measured separately, if ever.
        pub minutes: u32,
        agenda: &'static str,
    }

    pub const MAX_USEFUL_MINUTES: u32 = 0;

    pub fn agenda(&self) -> &str { self.agenda }

    pub fn expense<T>(&self, receipt: T) -> String
    where T: core::fmt::Display {
        format!("{} minutes: {receipt}", self.minutes)
    }
}

let meeting = Meeting::new(60, "schedule another meeting");
assert_eq!(meeting.agenda(), "schedule another meeting");
assert_eq!(meeting.expense("biscuits"), "60 minutes: biscuits");
```

## Declining the complimentary derives

Default derives require all fields and the parent to implement `Debug` and
`Clone`. Put `plain` first to opt out and choose your own traits. Do not add
`Debug` or `Clone` yourself in the default mode; Rust dislikes duplicate forms.

```rust
use class_warfare::class;

class! {
    plain
    #[derive(Debug)]
    class UniqueEmployee {
        pub badge: core::sync::atomic::AtomicU32,
    }

    pub fn badge_number(&self) -> u32 {
        self.badge.load(core::sync::atomic::Ordering::Relaxed)
    }
}

let employee = UniqueEmployee::new(core::sync::atomic::AtomicU32::new(42));
assert_eq!(employee.badge_number(), 42);
```

Class and field documentation and ordinary attributes are forwarded to the
generated struct and fields. Conditional compilation should wrap the **whole
macro invocation** (or its enclosing module); `#[cfg]` on a class or field does
not automatically remove the accompanying generated implementations or
constructor arguments. Representation and derive attributes follow Rust's
usual rules.

## The fine print, now with fewer lawyers

- This is embedded composition and reference coercion. It does not create a
  language-level subtype relationship. `Dog` and `Animal` remain distinct types.
- Child methods can shadow parent methods. Calls through `&Animal` still run
  `Animal`'s implementation. There are no virtual methods; use traits and
  `dyn Trait` when you need runtime dispatch.
- `&Child` and `&mut Child` can coerce through the parent chain in ordinary
  function arguments. Owned children do not coerce into owned parents. Use
  `into_super()` or `HasParent::into_parent()` to recover the direct parent;
  child-only fields are dropped. Implementing `Drop` on a child prevents this
  generated move-out operation from compiling.
- Parent traits, operators, associated functions, and associated constants
  are not automatically implemented or re-exported by children. Generic
  `T: SomeTrait` bounds also do not get satisfied by `Deref` magic.
- Consuming parent methods cannot move a parent out through `Deref`. Extract
  it explicitly before calling those methods.
- Constructors take fields in declaration order. Derived constructors take
  the already constructed parent first. Nothing calls a hidden initializer.
- `new`, `super_class`, `super_cast`, `super_cast_mut`, and `into_super` are
  reserved where generated. Trait implementations generated by the macro must
  not be implemented a second time for the same class.
- Parent types may use qualified paths and concrete generic arguments, such as
  `some_module::Parent<u32>`. Classes themselves cannot declare generic type
  parameters or lifetimes; use ordinary Rust structs for those. Methods can.
- Empty classes are supported. Multiple inheritance is left to committees,
  which are famously good at resolving ambiguity.
- The macro supplies no validation. Put domain rules in your methods or wrap
  the generated constructor in a validating factory. Public fields and
  `super_class` are directly mutable by design.

For example, a parent trait implementation does **not** satisfy a trait bound
on its child. The compiler is immune to claims of nepotism:

```compile_fail,E0277
use class_warfare::class;

trait Qualified {}
class! { class Employee {} }
impl Qualified for Employee {}
class! { class Manager extends Employee {} }

fn hire<T: Qualified>(_: T) {}
hire(Manager::new(Employee::new()));
```

Parent delegation also preserves Rust's field privacy across modules:

```compile_fail,E0616
mod vault {
    use class_warfare::class;
    class! { class Account { secret: u32 } }
}

use class_warfare::class;
class! { class Executive extends vault::Account {} }
let boss = Executive::new(vault::Account::new(42));
let confidential = boss.secret;
```

## Try it

From the repository root:

```text
cargo run
cargo run -p class-warfare --example office_politics
cargo run -p class-warfare --example family_business
cargo test --workspace
cargo doc -p class-warfare --no-deps
```

The integration suite exercises reference coercion, mutation, static method
shadowing, parent extraction, drop behavior, visibility, attributes, generic
methods, and use from a `no_std` consumer. The README examples are also doctests.

Licensed under MIT. The inheritance tax is zero.
