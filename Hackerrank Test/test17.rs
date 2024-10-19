use std::collections::HashMap;

fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for &bird in &arr {
        *count.entry(bird).or_insert(0) += 1;
    }
    
    let max_count = count.values().cloned().max().unwrap();
    let mut result = i32::MAX;

    for (&bird, &c) in &count {
        if c == max_count && bird < result {
            result = bird;
        }
    }
    
    result
}

fn main() {
    let mut arr_count = String::new();
    std::io::stdin().read_line(&mut arr_count).unwrap();
    let arr_count: usize = arr_count.trim().parse().unwrap();

    let mut arr = String::new();
    std::io::stdin().read_line(&mut arr).unwrap();
    let arr: Vec<i32> = arr.split_whitespace()
                           .map(|s| s.parse().unwrap())
                           .collect();

    println!("{}", migratory_birds(arr));
}
