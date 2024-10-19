use std::io;

fn divisible_sum_pairs(n: usize, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_line: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = first_line[0];
    let k: i32 = first_line[1] as i32;

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = divisible_sum_pairs(n, k, &ar);
    println!("{}", result);
}
