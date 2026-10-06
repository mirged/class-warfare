#![no_std]
#![allow(missing_docs)]

// No std imports or prelude are available to these macro expansions.
// Cargo supplies the test harness when running this integration target.
use class_warfare::{HasParent, class};

class! {
    class Counter { pub value: u32 }
    pub fn increment(&mut self) { self.value += 1; }
}

class! {
    class FancyCounter extends Counter { pub glitter: bool }
}

#[test]
fn generated_code_needs_only_core() {
    let mut counter = FancyCounter::new(Counter::new(41), true);
    counter.increment();
    assert_eq!(counter.value, 42);
    assert!(counter.glitter);
    assert_eq!(counter.parent().value, 42);
    assert_eq!(counter.into_parent().value, 42);
}
