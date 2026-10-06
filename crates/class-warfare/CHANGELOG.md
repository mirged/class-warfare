# Changelog

## 0.1.0

- Liberated `class!` from `src/main.rs` and gave it its own office.
- Preserved the original public classes, constructors, and parent casts.
- Added field visibility and attributes, arbitrary inherent items, and qualified parent types.
- Added `plain` mode for custom derives and non-cloneable fields.
- Added `HasParent`, parent extraction, `AsRef`, and `AsMut`.
- Added dependency-free `no_std` support, executable examples, and behavioral tests.
- Clarified that reference coercion and static method shadowing are not virtual inheritance.
