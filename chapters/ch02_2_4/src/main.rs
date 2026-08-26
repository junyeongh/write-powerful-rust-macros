use std::ops::{Add, Sub};

enum Currency {
    Euro,
    Dollar,
}

impl From<&str> for Currency {
    fn from(value: &str) -> Self {
        if value.contains("euro") {
            Currency::Euro
        } else {
            Currency::Dollar
        }
    }
}
impl Currency {
    fn calculate(&self, amount: u32) -> u32 {
        match self {
            Currency::Euro => amount,
            Currency::Dollar => amount * 2,
        }
    }
}

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
    // (give $amount:literal $currency:literal to $name:ident) => {
    //     let curr: Currency = $currency.into();
    //     $name.deposite(curr.calculate($amount));
    //     // $name.deposite($amount);
    // };
    (give $amount:literal euro to $name:ident) => {
        let curr: Currency = Currency::Euro;
        $name.deposite(curr.calculate($amount));
        // $name.deposite($amount);
    };
    (give $amount:literal dollar to $name:ident) => {
        let curr: Currency = Currency::Dollar;
        $name.deposite(curr.calculate($amount));
        // $name.deposite($amount);
    };

    (take $amount:literal $currency:literal from $name:ident) => {
        let curr: Currency = $currency.into();
        $name.withdraw(curr.calculate($amount));
        // $name.withdraw($amount);
    };

    (give $amount:literal $currency:literal from $giver:ident to $receiver:ident) => {
        let curr: Currency = $currency.into();
        $giver.withdraw(curr.calculate($amount));
        // $giver.withdraw($amount);
        $receiver.deposite(curr.calculate($amount));
        // $receiver.deposite($amount);
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

    // exchange!(give 20 "euro" to the_poor);
    exchange!(give 20 euro to the_poor);
    exchange!(give 20 dollar to the_poor);
    exchange!(take 10 "euro" from the_rich);
    exchange!(give 30 "euro" from the_rich to the_poor);

    println!("Poor: {the_poor:?}, Rich: {the_rich:?}");
}
