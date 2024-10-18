use std::io;

fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        primary_diagonal += arr[i][i];
        secondary_diagonal += arr[i][n - i - 1];
    }

    (primary_diagonal - secondary_diagonal).abs()
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut arr = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        arr.push(row);
    }

    let result = diagonal_difference(arr);
    println!("{}", result);
}
