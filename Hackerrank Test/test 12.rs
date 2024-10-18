use std::io;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return String::from("NO");
    }
    if (x2 - x1) % (v1 - v2) == 0 {
        return String::from("YES");
    }
    String::from("NO")
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line");
    let first_multiple_input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let (x1, v1, x2, v2) = (first_multiple_input[0], first_multiple_input[1], first_multiple_input[2], first_multiple_input[3]);
    let result = kangaroo(x1, v1, x2, v2);

    println!("{}", result);
}
