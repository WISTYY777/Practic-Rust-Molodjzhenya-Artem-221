use std::io;

fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);
    from_front.min(from_back)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let p: i32 = input.trim().parse().unwrap();

    let result = page_count(n, p);
    println!("{}", result);
}
