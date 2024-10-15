fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }
    let n = n.rem_euclid(len as isize) as usize;
    let (first, second) = s.split_at(len - n);
    format!("{}{}", second, first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];
        shifts
            .iter()
            .for_each(|(n, exp)| {
                assert_eq!(
                    rotate(s.to_string(), *n), 
                    exp.to_string()
                )
            });
    }
}

fn main() {
    let input = "abcdefgh".to_string();
    let shifts = 2;
    let result = rotate(input, shifts);
    println!("{}", result);
}
