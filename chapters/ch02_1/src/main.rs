macro_rules! my_vec {
    // matcher-transcriber pairs
    () => [
        Vec::new()
    ];

    (make an empty vec) => (
        Vec::new()
    );

    // Non-empty matchers
    // meaning `match any single Rust expression (expr)`
    // then, bind it to a value (macro variable) x
    {$x:expr} => { // x: macro variable
        { // this additional curly braces
          // for `let` statement e.g., `let a_number_vec = my_vec!(1)`
            let mut v = Vec::new();
            v.push($x);
            v
        }
    };

    //  [$($x:expr),+] => (
    [
        $($x:expr),+
        $(,)?
    ] => (
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )+
            v
        }
    );
}

fn main() {
    let empty_vec: Vec<i32> = my_vec!();
    println!("{:?}", empty_vec); // output: []

    let another_empty_vec: Vec<i32> = my_vec!(make an empty vec);
    println!("{:?}", another_empty_vec); // output: []

    let one_number: Vec<i32> = my_vec!(1);
    println!("{:?}", one_number); // output: [1]

    let three_numbers: Vec<i32> = my_vec![1, 2, 3];
    println!("{:?}", three_numbers); // output: [1, 2, 3]

    let one_number_with_trailing_comma: Vec<i32> = my_vec!(1,);
    println!("{:?}", one_number_with_trailing_comma); // output: [1]

    let three_numbers_with_trailing_comma: Vec<i32> = my_vec![1, 2, 3,];
    println!("{:?}", three_numbers_with_trailing_comma); // output: [1, 2, 3]
}
