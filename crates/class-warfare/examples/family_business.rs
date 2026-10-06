//! Reusable class syntax, ordinary traits, and an extremely literal family business.
use class_warfare::class;

class! {
    /// A business with a string attached.
    class Business {
        /// The sign over the door.
        pub name: &'static str,
        /// Revenue, before the accountant starts crying.
        pub coins: u32,
    }

    /// Process one legally uncomplicated payment.
    pub fn earn(&mut self, coins: u32) { self.coins += coins; }
}

class! {
    /// A business with cheese attached.
    class Pizzeria extends Business {
        /// Number of controversial toppings.
        pub pineapples: u32,
    }
}

trait Pitch {
    fn pitch(&self) -> String;
}

impl Pitch for Business {
    fn pitch(&self) -> String {
        format!("{}: We sell things. Revolutionary.", self.name)
    }
}

// Trait behavior is an explicit choice. Deref doesn't inherit trait implementations.
impl Pitch for Pizzeria {
    fn pitch(&self) -> String {
        format!(
            "{}: {} pineapples, one ongoing diplomatic incident.",
            self.name, self.pineapples
        )
    }
}

fn main() {
    let mut shop = Pizzeria::new(Business::new("Slice of Inheritance", 0), 3);
    shop.earn(20);
    let presenters: [&dyn Pitch; 2] = [shop.super_cast(), &shop];
    for presenter in presenters {
        println!("{}", presenter.pitch());
    }
    println!(
        "Revenue: {} coins. The parent company approves.",
        shop.coins
    );
}
