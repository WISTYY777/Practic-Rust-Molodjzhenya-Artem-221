#[derive(PartialEq)]
enum MyEnum {
    Foo,
    Bar,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Hello { id: i32 },
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants"),
    }
}

// Тест 1
#[test]
fn test_1() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success!");
}


// Тест 2
#[test]
fn test_2() {
    let test_cases = vec!["abc", "aBc", "A", "Z"];
    for ab in test_cases {
        assert!(ab.chars().all(char::is_alphabetic), "{} must be alphabetic", ab);
    }

    let invalid_cases = vec!["abc1", "123", "abc!", " "];
    for ab in invalid_cases {
        assert!(!ab.chars().all(char::is_alphabetic), "{} must not be alphabetic", ab);
    }
}


// Тест 3
#[test]
fn test_3() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo {
            count += 1;
        }
    }

    assert_eq!(count, 2);
    println!("Success!");
}

// Тест 4
#[test]
fn test_4() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and {:?}", i);
        println!("Success!");
    }
}

// Тест 5
#[test]
fn test_5() {
    enum Foo {
        Bar(u8),
    }

    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}

// Тест 6
#[test]
fn test_6() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Qux(10);

    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

// Тест 7
#[test]
fn test_7() {
    let age = Some(30);
    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}

fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Тест 8
#[test]
fn test_8() {
    let p = Point { x: 0, y: 20 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// Тест 9
#[test]
fn test_9() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id } if (3..=7).contains(&id) => println!("Found an id in range [3, 7]"),
        Message::Hello { id: newid } if (10..=12).contains(&newid) => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
        Message::Quit => println!("Received Quit message"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(content) => println!("Writing: {}", content),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}



// Тест 10
#[test]
fn test_10() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

// Тест 11
#[test]
fn test_11() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

// Тест 12
#[test]
fn test_12() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
    }
}

// Тест 13
#[test]
fn test_13() {
    let n = 3;
    match n {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
}

// Тест 14
#[test]
fn test_14() {
    let n = 5;
    match n {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4 => println!("four"),
        _ => println!("other"),
    }
}

// Тест 15
#[test]
fn test_15() {
    let n = 6;
    match n {
        1..=5 => println!("between 1 and 5"),
        6 => println!("six"),
        _ => println!("other"),
    }
}
