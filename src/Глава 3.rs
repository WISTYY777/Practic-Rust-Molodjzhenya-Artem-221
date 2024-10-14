//Глава 3
#[test]//Тест 1
fn main() {
    let x: i32 = 5; 
    let y: i32; 

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]//Тест 2
fn main() {
    let mut x = 1;
    x += 2;

    
    assert_eq!(x, 3);
    println!("Success!");
}

#[test]//Тест 3
fn main() {
    let x: i32 = 10;
    let y: i32 = 20;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

#[test]//Тест 4
fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}
//другий варіант вирішення
fn main() {
    let x = define_x();
    println!("{:?}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}

#[test]//Тест 5
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

 #[test]//Тест 6
fn main() {
    let mut x: i32 = 1;
    x = 7;
    let x = x;
    let y = 4;
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

#[test]//Тест 7 
fn main() {
    let _x = 1;
}

 #[test]//Тест 8
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

 #[test]//Тест 9
fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    println!("Success!");
}
