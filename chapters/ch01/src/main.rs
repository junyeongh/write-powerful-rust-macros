use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    first_name: String,
    last_name: String,
}

fn full_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

fn main() {
    let r = Request {
        first_name: "Junyeong".to_string(),
        last_name: "Heo".to_string(),
    };
    dbg!(full_name(&r.first_name, &r.last_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let actual: Request =
            serde_json::from_str("{\"first_name\": \"Test\", \"last_name\": \"McTest\"}")
                .expect("deserialize to work");

        assert_eq!(actual.first_name, "Test".to_string());
        assert_eq!(actual.last_name, "McTest".to_string());
    }
}
