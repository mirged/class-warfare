//! An inheritance hierarchy with precisely as much competence as a real office.
use class_warfare::{HasParent, class};

class! {
    /// The person who actually does the work.
    class Employee {
        /// The name on the badge.
        pub name: String,
        /// Meetings that could have been emails.
        pub meetings: u32,
    }

    /// Issue a completely accurate status update.
    pub fn status(&self) -> &'static str { "Doing the work." }

    /// A promotion in the calendar department.
    pub fn invite_to_meeting(&mut self) { self.meetings += 1; }
}

class! {
    /// The person who makes spreadsheets about the work.
    class Manager extends Employee {
        /// Direct reports, indirect suffering.
        pub reports: u32,
    }

    /// Shadow the parent's status, as management often does.
    pub fn status(&self) -> &'static str { "Taking credit for the work." }
}

class! {
    /// The person who makes slides about the spreadsheets.
    class Executive extends Manager {
        /// The approved vocabulary budget.
        pub buzzwords: u32,
    }

    /// Announce a strategically meaningless initiative.
    pub fn strategize(&self) {
        println!("{}: Let's synergize the borkchain. Budget: {} buzzwords.",
            self.name, self.buzzwords);
    }
}

fn audit(employee: &Employee) {
    println!(
        "Audit of {}: {} {} meetings on record.",
        employee.name,
        employee.status(),
        employee.meetings
    );
}

fn main() {
    let mut boss = Executive::new(
        Manager::new(Employee::new("Sir Spreadsheet III".into(), 0), 12),
        9000,
    );
    boss.invite_to_meeting();
    boss.invite_to_meeting();
    boss.strategize();
    println!("Management says: {}", boss.status());
    audit(&boss);
    println!(
        "The direct parent supervises {} people.",
        boss.parent().reports
    );

    let manager = boss.into_super();
    let employee = manager.into_parent();
    println!(
        "After restructuring, {} is still {}",
        employee.name,
        employee.status()
    );
}
