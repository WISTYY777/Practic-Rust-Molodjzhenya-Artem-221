fn rotate(s: String, n: isize) -> String {
    let length = s.len();
    if length == 0 {
        return s;
    }
    let n = (n % length as isize + length as isize) % length as isize; // Рахуємо зміщення в межах довжини рядка
    let n = n as usize;
    let (start, end) = s.split_at(length - n);
    format!("{}{}", end, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        let word = "abcdefgh";
        let cases = [
            (0, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (8, "abcdefgh"), 
        ];

        for (shift, expected) in cases.iter() {
            assert_eq!(rotate(word.to_string(), *shift), *expected);
        }
    }
}

fn main() {
    let word = "abcdefgh".to_string();
    let shift = 2;
    println!("{}", rotate(word, shift));
}
