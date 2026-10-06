// The class engine has unionized and moved into its own crate.
use class_warfare::class;

// =========================================================================
// DEFINING OUR CLASS HIERARCHY
// =========================================================================

// Level 1: Base Class
class! {
    class Animal {
        pub name: String,
        pub age: u32,
    }

    // Static / Factory Method
    pub fn create_stray(name: &str) -> Self {
        Self::new(name.to_string(), 1)
    }

    // Immutable Instance Method
    pub fn speak(&self) {
        println!("{}: [Generic animal sound]", self.name);
    }

    // Mutable Instance Method
    pub fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday, {}! Now {} years old.", self.name, self.age);
    }
}

// Level 2: Derived Class (Dog extends Animal)
class! {
    class Dog extends Animal {
        pub breed: String,
        pub bork_count: u32,
    }

    pub fn bark(&mut self) {
        self.bork_count += 1;
        println!("{}: WOOF! (Total barks: {})", self.name, self.bork_count);
    }
}

// Level 3: Multi-Level Inheritance (GuardDog extends Dog extends Animal)
class! {
    class GuardDog extends Dog {
        pub security_clearance: u8,
    }

    pub fn patrol(&self) {
        println!("{}: Patrolling compound. Security level: {}.", self.name, self.security_clearance);
    }
}

// =========================================================================
// POLYMORPHISM IN ACTION
// =========================================================================

// This function knows about Animal. Deref coercion accepts child references;
// method dispatch here still uses Animal's implementation.
fn inspect_animal(animal: &Animal) {
    println!("\n[VET INSPECTION]");
    println!("  Name: {}", animal.name);
    println!("  Age : {}", animal.age);
    animal.speak();
}

// =========================================================================
// EXECUTION
// =========================================================================

fn main() {
    println!("=== CLASS WARFARE: THE BARKING DEPARTMENT ===");
    println!("=== 1. BASE CLASS DEMO ===");
    let mut cat = Animal::create_stray("Whiskers");
    cat.speak();
    cat.celebrate_birthday();

    println!("\n=== 2. INHERITANCE & MUTABILITY DEMO ===");
    // Dog takes its parent Animal as the first argument in new()
    let mut doggo = Dog::new(
        Animal::new("Rex".to_string(), 3),
        "German Shepherd".to_string(),
        0,
    );

    // Call child method
    doggo.bark();

    // Call inherited parent method
    doggo.speak();

    // Mutate parent state directly from the child
    doggo.celebrate_birthday();

    println!("\n=== 3. MULTI-LEVEL INHERITANCE (Grandchild) ===");
    let guard_dog = GuardDog::new(
        Dog::new(
            Animal::new("Cerberus".to_string(), 5),
            "Doberman".to_string(),
            10,
        ),
        5, // clearance level
    );

    // GuardDog -> Dog method
    guard_dog.patrol();

    // GuardDog -> Animal field via 2-tier Deref chain!
    println!("Cerberus belongs to breed: {}", guard_dog.breed);
    println!("Cerberus age: {}", guard_dog.age);

    println!("\n=== 4. REFERENCE COERCION (The family discount) ===");
    // Pass Cat (&Animal)
    inspect_animal(&cat);

    // Pass Dog (&Dog auto-coerces to &Animal)
    inspect_animal(&doggo);

    // Pass GuardDog (&GuardDog coerces to &Dog, which coerces to &Animal!)
    inspect_animal(&guard_dog);
}
