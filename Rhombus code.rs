fn main() {
    const SIZE: usize = 5;
    for i in 0..SIZE * 2 - 1 {
        let n = if i < SIZE { i } else { SIZE * 2 - 2 - i };
        print!("{:width$}", "", width = SIZE - n - 1);
        for _ in 0..2 * n + 1 {
            print!("*");
        }
        println!();
    }
}
