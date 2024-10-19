use std::collections::HashMap;
use std::io;

fn sock_merchant(n: usize, ar: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }
    counts.values().map(|&count| count / 2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = sock_merchant(n, ar);
    println!("{}", result);
}
