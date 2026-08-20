// #![feature(trace_macros)]
// #![feature(log_syntax)]

use crate::greetings::base_greeting_fn;

#[macro_use]
mod greetings;

fn main() {
    let _greet = greeting!("Junyeong", "Hey there");
    println!("{}", _greet);

    let _greet_with_default = greeting!("Junyeong");
    println!("{}", _greet_with_default);
}

// #![feature(trace_macros)]
// #![feature(log_syntax)]

// fn main() {
//     trace_macros!(true);

//     let _greet = greeting!("Junyeong", "Hey there");
//     // println!("{}", _greet);

//     let _greet_with_default = greeting!("Junyeong");
//     // println!("{}", _greet_with_default);

//     let _greet_with_default_test = greeting!(test "Junyeong");

//     trace_macros!(false);
// }
