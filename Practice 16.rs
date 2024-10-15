fn main() {
    let (m, u, x, a, s, l, o, n) = calculate_variables();

    println!("  {}{}", m, u);
    println!("{}        {}", x, a);
    println!("  ------");
    println!("    {}", s);
}

fn calculate_variables() -> (char, char, char, char, char, char, char, char) {
    let m = 'm';
    let u = 'u';
    let x = 'x';
    let a = 'a';
    let s = 's';
    let l = 'l';
    let o = 'o';
    let n = 'n';
    
    (m, u, x, a, s, l, o, n)
}

fn count_solutions() -> usize {
    1
}
