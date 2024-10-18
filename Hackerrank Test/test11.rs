use std::io;

fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {
    let apple_count = apples.iter().filter(|&&apple| (a + apple).between(s, t)).count();
    let orange_count = oranges.iter().filter(|&&orange| (b + orange).between(s, t)).count();
    println!("{}", apple_count);
    println!("{}", orange_count);
}

trait Between {
    fn between(self, low: Self, high: Self) -> bool;
}

impl Between for i32 {
    fn between(self, low: Self, high: Self) -> bool {
        self >= low && self <= high
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line");
    let first_multiple_input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (s, t) = (first_multiple_input[0], first_multiple_input[1]);

    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    let second_multiple_input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (a, b) = (second_multiple_input[0], second_multiple_input[1]);

    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    let _ = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    let apples: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    let oranges: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    count_apples_and_oranges(s, t, a, b, apples, oranges);
}
