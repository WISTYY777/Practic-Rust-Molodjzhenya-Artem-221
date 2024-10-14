#[test] // Тест 1
fn test1() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

#[test] // Тест 2
fn test2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test] // Тест 3
fn test3() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test] // Тест 4
fn test4() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}

#[test] // Тест 5
fn test5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}

#[test] // Тест 6
fn test6() {
    let mut _x: i32 = 1;
    _x = 7;
    let _x = _x;
    let _y = 4;
    let _y = "I can also be bound to text!";

    println!("Success!");
}

#[test] // Тест 7
fn test7() {
    let _x = 1;
}

#[test] // Тест 8
fn test8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

#[test] // Тест 9
fn test9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    println!("Success!");
}
