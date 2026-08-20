trait NameRule<T> {
    fn new(name: &str) -> Result<T, String>;
    fn get_value(&self) -> &String;
}

struct FirstName {
    value: String,
}
struct LastName {
    value: String,
}

struct Age {
    value: i32,
}
struct Pay {
    value: i32,
}

macro_rules! generate_new_instance {
    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            pub fn new(value: $return_type) -> $struct_type {
                $struct_type { value }
            }
        }
    };
}

macro_rules! generate_get_value {
    // ($struct_type:ident) => {
    //     impl $struct_type {
    //         pub fn get_value(&self) -> &String {
    //             &self.value
    //         }
    //     }
    // };
    ($struct_type:ident) => {
        generate_get_value!($struct_type, String); // call itself
    };

    ($struct_type:ident, $return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    }; // impl

       // ($struct_type:ident, $return_type:ty) => {
       //     impl NameRule<$struct_type> for $struct_type {
       //         fn get_value(&self) -> &$return_type {
       //             &self.value
       //         }
       //     }
       // };
}

// macro_rules! count {
//     ($val:expr) => {
//         if $val == 1 { 1 } else { count!($val - 1) }
//     };
// }
// count!(5);

impl FirstName {
    pub fn new(name: &str) -> Result<FirstName, String> {
        if name.len() < 2 {
            Err("Name should be at least two characters".to_string())
        } else {
            Ok(FirstName {
                value: name.to_string(),
            })
        }
    }
}
generate_get_value!(FirstName);

impl NameRule<LastName> for LastName {
    fn new(name: &str) -> Result<LastName, String> {
        if name.len() < 2 {
            Err("Name should be at least two characters".to_string())
        } else {
            Ok(LastName {
                value: name.to_string(),
            })
        }
    }

    fn get_value(&self) -> &String {
        &self.value
    }
}
// generate_get_value!(LastName);

generate_new_instance!(Age, i32);
generate_new_instance!(Pay, i32);
generate_get_value!(Age, i32);
generate_get_value!(Pay, i32);

fn calculate_raise(first_name: FirstName, last_name: LastName, age: Age, current_pay: Pay) -> i32 {
    println!(
        "{}, {} ({}) - {}",
        first_name.get_value(),
        last_name.get_value(),
        age.get_value(),
        current_pay.get_value()
    );
    if first_name.get_value() == "Sam" && last_name.get_value() == "Smith" {
        *current_pay.get_value() + 1000
    } else {
        *current_pay.get_value()
    }
}

fn main() {
    let first_raise = calculate_raise(
        FirstName::new("Smith").unwrap(),
        LastName::new("Sam").unwrap(),
        Age::new(20),
        Pay::new(1000),
    );
    println!("{}", first_raise);

    let second_raise = calculate_raise(
        FirstName::new("Sam").unwrap(),
        LastName::new("Smith").unwrap(),
        Age::new(1000),
        Pay::new(20),
    );
    println!("{}", second_raise);
}
