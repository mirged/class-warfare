#![allow(missing_docs)]

use class_warfare::{HasParent, class};

class! {
    #[derive(PartialEq, Eq)]
    class Animal {
        pub name: String,
        pub age: u32,
    }

    pub const SPECIES: &'static str = "uncategorized noise generator";

    pub fn create_stray(name: &str) -> Self { Self::new(name.to_owned(), 1) }
    pub fn speak(&self) -> &'static str { "generic noise" }
    pub fn birthday(&mut self) { self.age += 1; }
}

class! {
    #[derive(PartialEq, Eq)]
    class Dog extends Animal {
        pub borks: u32,
    }

    pub fn speak(&self) -> &'static str { "bork" }
    pub fn bark(&mut self) { self.borks += 1; }
}

class! {
    class GuardDog extends Dog {
        pub clearance: u8,
    }
}

fn guard() -> GuardDog {
    GuardDog::new(Dog::new(Animal::new("Cerberus".into(), 5), 0), 7)
}

#[test]
fn constructors_factories_and_default_derives() {
    let stray = Animal::create_stray("Whiskers");
    assert_eq!(stray.name, "Whiskers");
    assert_eq!(stray.age, 1);
    assert_eq!(stray.clone(), stray);
    assert!(format!("{stray:?}").contains("Whiskers"));
    assert_eq!(Animal::SPECIES, "uncategorized noise generator");
    let dog = Dog::new(stray, 8);
    assert_eq!(dog.clone(), dog);
    assert_eq!(dog.borks, 8);
}

#[test]
fn multilevel_fields_methods_and_mutable_coercion() {
    fn inspect(animal: &Animal) -> (&str, u32) {
        (&animal.name, animal.age)
    }
    fn vaccinate(animal: &mut Animal) {
        animal.age += 1;
    }

    let mut dog = guard();
    dog.bark();
    dog.birthday();
    vaccinate(&mut dog);
    assert_eq!(inspect(&dog), ("Cerberus", 7));
    assert_eq!(dog.borks, 1);
    assert_eq!(dog.clearance, 7);
    dog.age = 9;
    assert_eq!(dog.super_class.super_class.age, 9);
}

#[test]
fn shadowing_is_static_and_parent_casts_bypass_it() {
    fn inspect(animal: &Animal) -> &str {
        animal.speak()
    }
    let dog = guard();
    assert_eq!(dog.speak(), "bork");
    assert_eq!(inspect(&dog), "generic noise");
    assert_eq!(dog.super_cast().super_cast().speak(), "generic noise");
}

#[test]
fn all_parent_helpers_address_the_direct_parent() {
    fn count<T: HasParent<Parent = Dog>>(child: &T) -> u32 {
        child.parent().borks
    }
    let mut dog = guard();
    dog.super_cast_mut().borks = 1;
    dog.parent_mut().borks += 1;
    let parent: &mut Dog = dog.as_mut();
    parent.borks += 1;
    let parent: &Dog = dog.as_ref();
    assert_eq!(parent.borks, 3);
    assert_eq!(count(&dog), 3);
    assert!(core::ptr::eq(dog.parent(), dog.super_cast()));
    let parent = dog.into_parent();
    assert_eq!(parent.borks, 3);
    assert_eq!(parent.into_super().name, "Cerberus");
}

#[test]
fn empty_classes_are_constructible() {
    class! { class Empty {} }
    class! { class AlsoEmpty extends Empty {} }
    let child = AlsoEmpty::new(Empty::new());
    let _: &Empty = &child;
    assert_eq!(core::mem::size_of_val(&child), 0);
    let _ = child.into_super();
}

mod accounting {
    #[derive(Debug, Clone)]
    pub struct Ledger<T> {
        pub balance: T,
    }
}

#[test]
fn parent_types_can_be_qualified_and_have_generic_arguments() {
    class! {
        class Auditor extends accounting::Ledger<u64> {
            pub receipts: u32,
        }
    }
    let mut auditor = Auditor::new(accounting::Ledger { balance: 40 }, 2);
    auditor.balance += 2;
    assert_eq!(auditor.super_cast().balance, 42);
    assert_eq!(auditor.receipts, 2);
}

mod payroll {
    use class_warfare::class;

    class! {
        #[derive(PartialEq, Eq)]
        pub(crate) class Paycheck {
            /// Confidential, mostly because it is embarrassing.
            amount: u32,
            pub(crate) memo: &'static str,
        }

        pub fn amount(&self) -> u32 { self.secret_amount() }
        fn secret_amount(&self) -> u32 { self.amount }
    }
}

#[test]
fn explicit_visibility_attributes_and_private_methods_work() {
    let pay = payroll::Paycheck::new(42, "pizza budget");
    assert_eq!(pay.amount(), 42);
    assert_eq!(pay.memo, "pizza budget");
    assert_eq!(pay, pay.clone());
}

#[test]
fn arbitrary_inherent_items_and_raw_identifiers_work() {
    class! {
        class Invoice {
            pub r#type: &'static str,
        }

        pub const TAX: u32 = 0;
        pub fn receipt<T>(&self, value: T) -> String
        where T: core::fmt::Display {
            format!("{}: {value}", self.r#type)
        }
        pub async fn file(&self) -> &str { self.r#type }
    }
    let invoice = Invoice::new("snacks");
    assert_eq!(invoice.receipt(42), "snacks: 42");
    assert_eq!(Invoice::TAX, 0);
    // Type-check the async method without adding an executor dependency.
    let _filing = invoice.file();
}

#[test]
fn plain_classes_accept_non_clone_and_non_debug_fields() {
    struct Badge(u32);
    class! { plain class Employee { pub badge: Badge } }
    class! { plain class Manager extends Employee { pub reports: u32 } }
    let manager = Manager::new(Employee::new(Badge(42)), 3);
    assert_eq!(manager.badge.0, 42);
    assert_eq!(manager.reports, 3);
    assert_eq!(manager.into_super().badge.0, 42);
}

#[test]
fn plain_mode_accepts_custom_derives_and_visibility() {
    class! {
        plain
        #[derive(Debug, PartialEq, Eq)]
        pub(crate) class Token { value: u8 }
    }
    class! {
        plain
        #[derive(Debug, PartialEq, Eq)]
        pub(crate) class Receipt extends Token {}
    }
    assert_eq!(Receipt::new(Token::new(7)), Receipt::new(Token::new(7)));
}

#[test]
fn extracting_parent_drops_child_fields_but_preserves_parent() {
    use std::{cell::Cell, rc::Rc};
    struct Tracked(Rc<Cell<u32>>);
    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    class! { plain class Parent { pub resource: Tracked } }
    class! { plain class Child extends Parent { pub resource: Tracked } }

    let parent_drops = Rc::new(Cell::new(0));
    let child_drops = Rc::new(Cell::new(0));
    let child = Child::new(
        Parent::new(Tracked(parent_drops.clone())),
        Tracked(child_drops.clone()),
    );
    let parent = child.into_super();
    assert_eq!(child_drops.get(), 1);
    assert_eq!(parent_drops.get(), 0);
    drop(parent);
    assert_eq!(parent_drops.get(), 1);
}
