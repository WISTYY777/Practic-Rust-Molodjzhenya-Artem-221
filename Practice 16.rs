use std::convert::TryInto;

fn check_solution(m: i32, u: i32, x: i32, a: i32, s: i32, l: i32, o: i32, n: i32) -> bool {
    let muxa = 1000 * m + 100 * u + 10 * x + a;
    let slon = 1000 * s + 100 * l + 10 * o + n;
    muxa * a == slon
}

fn permute(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
    if start == nums.len() {
        result.push(nums.clone());
    } else {
        for i in start..nums.len() {
            nums.swap(start, i);
            permute(nums, start + 1, result);
            nums.swap(start, i);
        }
    }
}

fn find_solutions() {
    let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut solution_count = 0;
    let mut results = Vec::new();

    permute(&mut digits, 0, &mut results);

    for combination in results {
        let [m, u, x, a, s, l, o, n]: [i32; 8] = combination.try_into().unwrap();
        if check_solution(m, u, x, a, s, l, o, n) {
            solution_count += 1;
            println!("Solution {}: Муха={}{}{}{} Слон={}{}{}{}", 
                solution_count, m, u, x, a, s, l, o, n);
        }
    }
    println!("Total solutions found: {}", solution_count);
}

fn main() {
    find_solutions();
}
