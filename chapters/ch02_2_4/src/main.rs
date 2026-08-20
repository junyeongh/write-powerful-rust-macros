use std::ops::{Add, Sub};

#[derive(Debug)]
struct Account {
    money: u32,
}

impl Account {
    fn deposite(&mut self, amount: u32) {
        self.money = self.money.add(amount);
    }

    fn withdraw(&mut self, amount: u32) {
        self.money = self.money.sub(amount);
    }
}

macro_rules! exchange {
    (give $amount:literal to $name:ident) => {
        $name.deposite($amount);
    };

    (take $amount:literal from $name:ident) => {
        $name.withdraw($amount);
    };

    (give $amount:literal from $giver:ident to $receiver:ident) => {
        $giver.withdraw($amount);
        $receiver.deposite($amount);
    };
}

fn main() {
    let mut my_account = Account { money: 10_000 };
    println!("{:?} - Initialized", my_account);

    Account::deposite(&mut my_account, 5000);
    println!("{:?} - After deposite 5000", my_account);

    Account::withdraw(&mut my_account, 3000);
    println!("{:?} - After withdraw 3000", my_account);

    let mut the_poor = Account { money: 0 };
    let mut the_rich = Account { money: 100_000 };

    exchange!(give 20 to the_poor);
    exchange!(take 10 from the_rich);
    exchange!(give 30 from the_rich to the_poor);

    println!("Poor: {the_poor:?}, Rich: {the_rich:?}");
}
