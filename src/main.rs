
mod fly {
    pub trait FlyBehavior {
        fn perform(&self);
    }
    pub struct FlyWithWings;
    impl FlyWithWings {
        pub fn new()->Self { FlyWithWings  }
    }
    impl FlyBehavior for FlyWithWings {
        fn perform(&self) {
            println!("I am flying by the wings !!!");
        }
    }

    pub struct FlyNoWay;
    impl FlyNoWay {
        pub fn new() -> Self {FlyNoWay}
    }
    impl FlyBehavior for FlyNoWay {
        fn perform(&self) {
            println!("I can't fly !!!");
        }
    }

    pub struct FlyJet;
    impl FlyJet {
        pub fn new() -> Self {FlyJet}
    }
    impl FlyBehavior for FlyJet {
        fn perform(&self) {
            println!("I am flying like a rocket !!!");
        }
    }
}

mod quack {
    pub trait QuackBehavior {
        fn perform(&self);
    }

    pub struct QuackAsDuck;
    impl QuackAsDuck {
        pub fn new() -> Self { QuackAsDuck  }
    }
    impl QuackBehavior for QuackAsDuck {
        fn perform(&self) {
            println!("Quack-quack!!!");
        }
    }

    pub struct MuteQuack;
    impl MuteQuack {
        pub fn new() -> Self { MuteQuack  }
    }
    impl QuackBehavior for MuteQuack {
        fn perform(&self) {
            println!("<<< Silence >>>");
        }
    }

    pub struct SqueakQuack;
    impl SqueakQuack {
        pub fn new() -> Self { SqueakQuack  }
    }
    impl QuackBehavior for SqueakQuack {
        fn perform(&self) {
            println!("Squeak-squeak !!!");
        }
    }
}

mod duck {
    use fly::*;
    use quack::*;

    pub enum DuckKind {
        MallardDuck,
        ModelDuck
    }
    pub struct Duck {
        name    : String,
        fly     :  Box<FlyBehavior>,
        quack   :  Box<QuackBehavior>,
    }

    impl Duck {
        pub fn new(kind : DuckKind) -> Duck {
            let duck = match kind {
                DuckKind::MallardDuck => Duck {
                    name : String::from("Mallard Duck"),
                    fly: Box::new(FlyWithWings::new()),
                    quack: Box::new(QuackAsDuck::new())
                },
                DuckKind::ModelDuck => Duck {
                    name : String::from("Model Duck"),
                    fly: Box::new(FlyNoWay::new()),
                    quack: Box::new(QuackAsDuck::new())
                },
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
        pub fn setFlyBehavior(&mut self, behavior : Box<FlyBehavior>) {
            self.fly = behavior;
        }
    }
}

use duck::{Duck, DuckKind};

fn main() {
    {
        let duck = Duck::new(DuckKind::MallardDuck);
        duck.fly();
        duck.quack();
    }
    {
        let mut duck = Duck::new(DuckKind::ModelDuck);
        duck.fly();
        duck.quack();
        duck.setFlyBehavior(Box::new(fly::FlyJet::new()));
        duck.fly();
    }
}
