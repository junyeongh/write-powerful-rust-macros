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
// generate_get_value!(FirstName);

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

macro_rules! generate_from {
    ($struct_type:ident) => {
        generate_from!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ty) => {
        impl From<$struct_type> for $return_type {
            fn from(f: $struct_type) -> Self {
                f.value
            }
        }
    };
}
generate_from!(Age, i32);

macro_rules! generate_newtypes_methods {
    ($struct_type:ident) => {
        generate_get_value!($struct_type, String);
        generate_from!($struct_type, String);
    };
    ($struct_type:ident, $return_type:ty) => {
        generate_get_value!($struct_type, $return_type);
        generate_from!($struct_type, $return_type);
    };
}
generate_newtypes_methods!(FirstName);

fn main() {
    let first_raise_first_name = FirstName::new("Smith").unwrap();
    let first_raise_last_name = LastName::new("Sam").unwrap();
    let first_raise_age = Age::new(20);
    // let first_raise_age = Age::from(20);
    let first_raise_pay = Pay::new(1000);
    let first_raise = calculate_raise(
        first_raise_first_name,
        first_raise_last_name,
        first_raise_age,
        first_raise_pay,
    );
    println!("{}", first_raise);

    let second_raise_first_name = FirstName::new("Sam").unwrap();
    let second_raise_last_name = LastName::new("Smith").unwrap();
    let second_raise_age = Age::new(1000);
    let second_raise_pay = Pay::new(20);
    let second_raise = calculate_raise(
        second_raise_first_name,
        second_raise_last_name,
        second_raise_age,
        second_raise_pay,
    );
    println!("{}", second_raise);
}
