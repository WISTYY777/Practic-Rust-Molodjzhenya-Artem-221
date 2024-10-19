use std::io;

fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let (mut min_count, mut max_count) = (0, 0);
    
    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }
    vec![max_count, min_count]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut scores_input = String::new();
    io::stdin().read_line(&mut scores_input).unwrap();
    let scores: Vec<i32> = scores_input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let result = breaking_records(scores);
    println!("{} {}", result[0], result[1]);
}

