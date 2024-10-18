use std::io;

fn mini_max_sum(arr: Vec<i64>) {
    let sum: i64 = arr.iter().sum();
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    
    let min_sum = sum - max;
    let max_sum = sum - min;

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    mini_max_sum(arr);
}

