fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_lowercase() {
            c.to_uppercase().next().unwrap()
        } else {
            c.to_lowercase().next().unwrap()
        })
        .collect()
}

fn main() {
    let text = "Доброго вечора ми з україни!!!";
    let result = swap_case(text);
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
