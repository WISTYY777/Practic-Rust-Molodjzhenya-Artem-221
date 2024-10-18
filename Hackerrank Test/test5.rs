use std::io;

fn plus_minus(arr: Vec<i32>) {
    let n = arr.len();
    let (mut positive_count, mut negative_count, mut zero_count) = (0, 0, 0);

    for &num in &arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    println!("{:.6}", positive_count as f64 / n as f64);
    println!("{:.6}", negative_count as f64 / n as f64);
    println!("{:.6}", zero_count as f64 / n as f64);
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    plus_minus(arr);
}

