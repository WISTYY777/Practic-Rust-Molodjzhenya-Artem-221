fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_lowercase().collect::<String>()
            }
        })
        .collect()
}

fn main() {
    let result = swap_case("Доброго вечора ми з україни!!!"); //Можливо поставити любий текст який вам сподобаєця (вибрав такий текст просто як привітання але думаю таких буде багато:))
    println!("{}", result); 
    println!("Результат зміни регістру: {}", result); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_case() {
        assert_eq!(swap_case("Привіт Світ"), "пРИВІТ сВІТ");
        assert_eq!(swap_case("Rust"), "rUST");
        assert_eq!(swap_case("123!@#"), "123!@#");
    }
}
