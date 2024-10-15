// Тест 1
#[test]
fn test1() {
    let _y: i32 = 5;

    println!("Success!");
}

// Тест 2
#[test]
fn test2() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

// Тест 3
#[test]
fn test3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// Тест 4
#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// Тест 5
#[test]
fn test5() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i8::checked_add(120, 7).unwrap();

    println!("{}, {}", v1, v2);
}

// Тест 6
#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}

// Тест 7
#[test]
fn test7() {
    let x = 1_000.000_1_f64;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

// Тест 8
#[test]
fn test8() {
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < 1e-10);

    println!("Success!");
}

// Тест 9
#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

// Тест 10
#[test]
fn test10() {
    use std::ops::{Range, RangeInclusive};

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// Тест 11
#[test]
fn test11() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(3 * 50 == 150);
    assert!((9.6_f64 / 3.2 - 3.0).abs() < 1e-10);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// Тест 12
#[test]
fn test12() {
    use std::mem::size_of_val;
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 1);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 3);

    println!("Success!");
}

// Тест 13
#[test]
fn test13() {
    let c1 = "中";
    print_char(c1.chars().next().unwrap());
}

fn print_char(c: char) {
    println!("{}", c);
}

// Тест 14
#[test]
fn test14() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    }
}

// Тест 15
#[test]
fn test15() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

// Тест 16
#[test]
fn test16() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() -> (i32, i32) {
    (2, 3)
}

// Тест 17
#[test]
fn test17() {
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

// Тест 18
#[test]
fn test18() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

// Тест 19
#[test]
fn test19() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

// Тест 20
#[test]
fn test20() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// Тест 21
#[test]
fn test21() {
    let result = print();
    assert_eq!(result, 0);
}

// Тест 22
#[test]
fn test22() {
    never_return();
}

// Тест 23
#[test]
fn test23() {
    let option = get_option(1);
    assert_eq!(option, Some(42));

    let option = get_option(0);
    assert!(option.is_none());
}

// Тест 24
#[test]
fn test24() {
    let b = true;

    let _v = match b {
        true => 1,
        false => {
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

// Тест 25
#[test]
fn test25() {
    let b = true;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

fn print() -> i32 {
    0
}

fn never_return() -> ! {
    loop {}
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42),
        _ => None,
    };

    never_return_fn()
}

fn never_return_fn() -> ! {
    loop {}
}
