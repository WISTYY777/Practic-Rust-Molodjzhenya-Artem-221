use std::io;

fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    let max_height = *candles.iter().max().unwrap();
    candles.iter().filter(|&&candle| candle == max_height).count() as i32
}

fn main() {
    let mut candles_count_temp = String::new();
    io::stdin().read_line(&mut candles_count_temp).unwrap();
    
    let candles_count: usize = candles_count_temp.trim().parse().unwrap();
    
    let mut candles_temp = String::new();
    io::stdin().read_line(&mut candles_temp).unwrap();
    
    let candles: Vec<i32> = candles_temp
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let result = birthday_cake_candles(candles);
    
    println!("{}", result);
}

