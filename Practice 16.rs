fn check_solution(m: i32, u: i32, x: i32, a: i32, s: i32, l: i32, o: i32, n: i32) -> bool {
    let muxa = 1000 * m + 100 * u + 10 * x + a;
    let slon = 1000 * s + 100 * l + 10 * o + n;
    muxa * a == slon
}

fn find_solutions() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut solution_count = 0;

    // Перебираємо всі можливі комбінації
    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x == m || x == u { continue; }
                for &a in &digits {
                    if a == m || a == u || a == x { continue; }
                    for &s in &digits {
                        if s == m || s == u || s == x || s == a { continue; }
                        for &l in &digits {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for &o in &digits {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for &n in &digits {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }
                                    
                                    if check_solution(m, u, x, a, s, l, o, n) {
                                        solution_count += 1;
                                        println!("Solution {}: ", solution_count);
                                        println!(" MUHA: {}{}{}{}", m, u, x, a);
                                        println!(" SLON: {}{}{}{}", s, l, o, n);
                                        println!();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Total solutions found: {}", solution_count);
}

fn main() {
    find_solutions();
}

