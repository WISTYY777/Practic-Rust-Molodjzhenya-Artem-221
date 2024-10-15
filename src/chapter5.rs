// Тест 1
#[test]
fn test1() {
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}

// Тест 2
#[test]
fn test2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// Тест 3
#[test]
fn test3() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("Hello world");
    s
}

// Тест 4
#[test]
fn test4() {
    let s = String::from("Hello World");
    print_str(&s);
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

// Тест 5
#[test]
fn test5() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

// Тест 6
#[test]
fn test6() {
    let mut s = String::from("Hello ");
    s.push_str("World!");
    println!("Success!");
}

// Тест 7
#[test]
fn test7() {
    let x = Box::new(5);
    let mut y = Box::new(5); 
    *y = 4;
    assert_eq!(*x, 5);
    println!("Success!");
}

// Тест 8
#[test]
fn test8() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    println!("The person's age from person struct is {}", person.age);
}

// Тест 9
#[test]
fn test9() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0.clone();
    println!("{:?}", t);
}

// Тест 10
#[test]
fn test10() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = (t.0.clone(), t.1.clone());
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}

// Тест 11
#[test]
fn test11() {
    let x = 5;
    let p = &x;
    println!("the memory address of x is {:p}", p);
}

// Тест 12
#[test]
fn test12() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
    println!("Success!");
}

// Тест 13
#[test]
fn test13() {
    let mut s = String::from("hello, ");
    borrow_object_immutable(&s);
    println!("Success!");
}

fn borrow_object_immutable(s: &String) {}

// Тест 14
#[test]
fn test14() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

// Тест 15
#[test]
fn test15() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    println!("{}", r1);
}

// Тест 16
#[test]
fn test16() {
    let c = '中';
    let r1 = &c;
    let r2 = c;
    assert_eq!(*r1, r2);
    println!("Success!");
}

// Тест 17
#[test]
fn test17() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    println!("{}", r1);
    println!("Success!");
}

// Тест 18
#[test]
fn test18() {
    let mut s = String::from("hello, ");
    borrow_object_mutable(&mut s);
    println!("Success!");
}

fn borrow_object_mutable(s: &mut String) {}

// Тест 19
#[test]
fn test19() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
}

// Тест 20
#[test]
fn test20() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
}

// Тест 21
#[test]
fn test21() {
    let c = 'A';
    let r1 = &c;
    let r2 = c;
    assert_eq!(*r1, r2);
    println!("Success!");
}
