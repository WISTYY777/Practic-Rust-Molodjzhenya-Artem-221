use std::io;

fn time_conversion(s: &str) -> String {
    let (time, period) = s.split_at(s.len() - 2);
    let mut parts: Vec<String> = time.split(':').map(|p| p.to_string()).collect();
    let hour: i32 = parts[0].parse().unwrap();

    if period == "AM" {
        if hour == 12 {
            parts[0] = "00".to_string();
        }
    } else if period == "PM" {
        if hour != 12 {
            let new_hour = hour + 12;
            parts[0] = new_hour.to_string();
        }
    }

    format!("{}:{}:{}", parts[0], parts[1], parts[2])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let result = time_conversion(input.trim());
    println!("{}", result);
}
