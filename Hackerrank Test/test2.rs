use std::io;

fn compare_triplets(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut scores = vec![0, 0];
    for i in 0..3 {
        if a[i] > b[i] {
            scores[0] += 1;
        } else if a[i] < b[i] {
            scores[1] += 1;
        }
    }
    scores
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = compare_triplets(a, b);
    println!("{}", result.iter().map(i32::to_string).collect::<Vec<String>>().join(" "));
}
