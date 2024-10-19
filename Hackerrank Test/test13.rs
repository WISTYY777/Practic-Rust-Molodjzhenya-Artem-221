use std::io;

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 { x } else { gcd(y, x % y) }
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm = |x, y| x * y / gcd(x, y);
    let lcm_a = a.iter().copied().reduce(lcm).unwrap_or(1);
    let gcd_b = b.iter().copied().reduce(gcd).unwrap();
    
    (lcm_a..=gcd_b).filter(|&i| gcd_b % i == 0 && a.iter().all(|&x| i % x == 0)).count() as i32
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (n, m) = (nums[0], nums[1]);

    let a: Vec<i32> = io::stdin().lines().next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let b: Vec<i32> = io::stdin().lines().next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{}", get_total_x(&a, &b));
}
