macro_rules! _curry {
    (|$first_arg:ident $(, $arg:ident)*| $function_body:expr) => {
        move |$first_arg| $function_body
    };
}

fn main() {
    println!("Hello, world!");
}
