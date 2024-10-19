#[test] // Тест 1
fn test1() {
    let s1 = "hello";
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}

#[test] // Тест 2
fn test2() {
    print!("hello world, ");
    print!("I am");
    print!("Sunface!");
}

#[derive(Debug)]
struct Structure(i32);

#[test] // Тест 3
fn test3() {
    println!("{} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3));
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[test] // Тест 4
fn test4() {
    let person = Person { name: "Sunface".to_string(), age: 18 };
    println!("{:?}", person);
}

#[derive(Debug)]
struct Deep(Structure);

#[test] // Тест 5
fn test5() {
    println!("Now {:?} will print!", Deep(Structure(7)));
}

use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
    }
}

#[test] // Тест 6
fn test6() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
    println!("Success!");
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

#[test] // Тест 7
fn test7() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}", v), "[1, 2, 3]");
    println!("Success!");
}

#[test] // Тест 8
fn test8() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    assert_eq!(format!("{1}{0}", 1, 2), "2112");
    assert_eq!(format!("{}{}", 1, 2), "2112");
    println!("Success!");
}

#[test] // Тест 9
fn test9() {
    println!("{argument}", argument = "test");
    assert_eq!(format!("{name}{}", 1, name = "2"), "21");
    assert_eq!(format!("{} {} {}", "a", 3, "b"), "a 3 b");
    println!("{abc} {1}", abc = "def", 2);
    println!("Success!");
}

#[test] // Тест 10
fn test10() {
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");
    assert_eq!(format!("Hello {:5}!", "x"), "Hello x    !");
    println!("Success!");
}

#[test] // Тест 11
fn test11() {
    println!("Hello {:<5}!", "x");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&!");
    println!("Success!");
}

#[test] // Тест 12
fn test12() {
    println!("Hello {:5}!", 5);
    println!("Hello {:+}!", 5);
    println!("Hello {:05}!", 5);
    println!("Hello {:05}!", -5);
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    println!("Success!")
}

#[test] // Тест 13
fn test13() {
    let v = 3.1415926;
    println!("{:.1$}", v, 4);
    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");
    println!("Success!");
}

#[test] // Тест 14
fn test14() {
    let s = "Hello, world!";
    println!("{0:.5}", s);
    assert_eq!(format!("Hello {:0$}!", 3, "abcdefg"), "Hello abc!");
    println!("Success!");
}

#[test] // Тест 15
fn test15() {
    assert_eq!(format!("{:b}", 27), "11011");
    assert_eq!(format!("{:o}", 27), "33");
    assert_eq!(format!("{:x}", 27), "1b");
    assert_eq!(format!("{:X}", 27), "1B");
    println!("{:x}!", 27);
    println!("{:#010b}", 27);
    println!("Success!");
}

fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}

#[test] // Тест 16
fn test16() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];
    for (name, score) in scores {
        println!("{name}: {:.1$}", score, precision);
    }
}
