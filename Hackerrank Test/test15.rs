use std::io;

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    (0..=s.len() - m as usize)
        .filter(|&i| s[i..i + m as usize].iter().sum::<i32>() == d)
        .count() as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut s_input = String::new();
    io::stdin().read_line(&mut s_input).unwrap();
    let s: Vec<i32> = s_input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut first_multiple_input = String::new();
    io::stdin().read_line(&mut first_multiple_input).unwrap();
    let mut first_multiple_iter = first_multiple_input.split_whitespace();
    let d: i32 = first_multiple_iter.next().unwrap().parse().unwrap();
    let m: i32 = first_multiple_iter.next().unwrap().parse().unwrap();

    let result = birthday(&s, d, m);
    println!("{}", result);
}
