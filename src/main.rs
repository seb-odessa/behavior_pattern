
mod fly {
    pub trait FlyBehavior {
        fn perform(&self);
    }
    pub struct FlyWithWings;
    impl FlyWithWings {
        pub fn new()->Self {
            FlyWithWings
        }
    }
    impl FlyBehavior for FlyWithWings {
        fn perform(&self) {
            println!("Fly with Wings!!!");
        }
    }
}

mod quack {
    pub trait QuackBehavior {
        fn perform(&self);
    }

    pub struct QuackAsDuck;

    impl QuackAsDuck {
        pub fn new() -> Self {
            QuackAsDuck
        }
    }

    impl QuackBehavior for QuackAsDuck {
        fn perform(&self) {
            println!("Quack!!!");
        }
    }
}

mod mallard {
    use fly::{FlyBehavior, FlyWithWings};
    use quack::{QuackBehavior, QuackAsDuck};

    pub struct MallardDuck {
        name    : String,
        fly     :  Box<FlyBehavior>,
        quack   :  Box<QuackBehavior>,
    }

    impl MallardDuck {
        pub fn new() -> MallardDuck {
            let duck = MallardDuck {
                name : String::from("Mallard Duck"),
                fly: Box::new(FlyWithWings::new()),
                quack: Box::new(QuackAsDuck::new())
            };
            println!("{} was created.", &duck.name);
            return duck;
        }
        pub fn fly(&self) {
            print!("{} perform: ", &self.name);
            self.fly.perform();
        }
        pub fn quack(&self) {
            print!("{} perform: ", &self.name);
            self.quack.perform();
        }
    }
}

use mallard::MallardDuck;

fn main() {
    let duck = MallardDuck::new();
    duck.fly();
    duck.quack();
}
