#[test] // Тест 1
fn test1() {
    if beverage == "lemonade" {
        println!("Success!");
        println!("Exercise Failed if printing out this line!");
    }
}

#[test] // Тест 2
fn test2() {
    assert_eq!("abc".as_bytes(), [96, 97, 98]);

    let v = vec![1, 2, 3];
    let ele = v.get(3).unwrap();

    let v = production_rate_per_hour(2);
    divide(15, 0);

    println!("Success!");
}

#[test] // Тест 3
fn test3() {
    fn divide(x:u8, y:u8) {
        println!("{}", x / y)
    }

    fn production_rate_per_hour(speed: u8) -> f64 {
        let cph: u8 = 221;
        match speed {
            1..=4 => (speed * cph) as f64,
            5..=8 => (speed * cph) as f64 * 0.9,
            9..=10 => (speed * cph) as f64 * 0.77,
            _ => 0 as f64,
        }
    }

    pub fn working_items_per_minute(speed: u8) -> u32 {
        (production_rate_per_hour(speed) / 60 as f64) as u32
    }
}

#[test] // Тест 4
fn test4() {
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>();
        let n2 = n2_str.parse::<i32>();
        Ok(n1.unwrap() * n2.unwrap())
    }

    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("t", "2");
    assert_eq!(result.is_err(), true);
    println!("Success!");
}

#[test] // Тест 5
fn test5() {
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
    }

    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}

#[test] // Тест 6
fn test6() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
}

#[test] // Тест 7
fn test7() {
    use std::num::ParseIntError;

    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().map(|n| n + 2)
    }

    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success!");
}

#[test] // Тест 8
fn test8() {
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str.parse::<i32>().and_then(|n1| {
            n2_str.parse::<i32>().map(|n2| n1 * n2)
        })
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);

    println!("Success!");
}

#[test] // Тест 9
fn test9() {
    use std::num::ParseIntError;

    type Res<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Res<i32>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}
