fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=f64::from(n).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let tests = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (16, false),
            (17, true),
            (18, false),
            (19, true),
            (20, false),
            (23, true),
            (24, false),
            (29, true),
        ];

        for (n, expected) in tests.iter() {
            assert_eq!(is_prime(*n), *expected);
        }
    }
}

fn main() {
    let number = 17;
    println!("Чи є {} простим числом? {}", number, is_prime(number));
}


