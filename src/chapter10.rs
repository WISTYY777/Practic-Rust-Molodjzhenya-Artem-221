struct A;          
struct S(A);       
struct SGen<T>(T); 

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

#[test] // Тест 1
fn test1() {
    reg_fn(S(A));          
    gen_spec_t(SGen(A));   
    gen_spec_i32(SGen(3)); 

    generic::<char>(SGen('a'));
    generic(SGen('b'));

    println!("Success!");
}

fn sum<T: std::ops::Add<Output = T> + Copy>(x: T, y: T) -> T {
    x + y
}

#[test] // Тест 2
fn test2() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}

struct Point<T> {
    x: T,
    y: T,
}

#[test] // Тест 3
fn test3() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
}

struct Val<T> {
    val: T,
}

impl Val<f64> {
    fn value(&self) -> &f64 {
        &self.val
    }
}

#[test] // Тест 4
fn test4() {
    let x = Val { val: 3.0 };
    let y = Val { val: "hello".to_string() };
    println!("{}, {}", x.value(), y.value());
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

#[test] // Тест 5
fn test5() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

struct Array<T, const N: usize> {
    data: [T; N],
}

#[test] // Тест 6
fn test6() {
    let arrays = [
        Array { data: [1, 2, 3] },
        Array { data: [1.0, 2.0, 3.0] },
        Array { data: [1, 2] },
    ];

    println!("Success!");
}

fn print_array<T: std::fmt::Debug>(arr: T) {
    println!("{:?}", arr);
}

#[test] // Тест 7
fn test7() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

#[test] // Тест 8
fn test8() {
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; 1]); 
    check_size([(); 1].map(|_| "hello你好".to_string()));  
    check_size(['中'; 1]);

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String {
        "I'm a good student".to_string()
    }
}
struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        "Hi, I'm your new teacher".to_string()
    }
    fn say_something(&self) -> String {
        "I'm not a bad teacher".to_string()
    }
}

#[test] // Тест 9
fn test9() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug)]
struct Seconds(i32);

#[test] // Тест 10
fn test10() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}

fn multiply<T: std::ops::Mul<Output = T> + Copy>(x: T, y: T) -> T {
    x * y
}

#[test] // Тест 11
fn test11() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}

struct Foo;
struct Bar;

struct FooBar;

struct BarFoo;

impl std::ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl std::ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}

#[test] // Тест 12
fn test12() {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

#[test] // Тест 13
fn test13() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    println!("{:?}", post);
    println!("{:?}", weibo);
}

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

#[test] // Тест 14
fn test14() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

#[test] // Тест 15
fn test15() {
    assert_eq!(sum(1, 2), 3);
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug> Pair<T> {
    fn debug(&self) {
        println!("{:?}, {:?}", self.x, self.y);
    }
}

#[test] // Тест 16
fn test16() {
    let pair = Pair::new(1, 2);
    pair.debug();
}

#[test] // Тест 17
fn test17() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(1), 2);
    assert_eq!(cacher.value(2), 3);

    println!("Success!");
}

#[test] // Тест 18
fn test18() {
    let some_u8_value = Some(5);
    let some_string_value = Some("a string");
    let absent_string_value: Option<&str> = None;

    if let Some(value) = some_u8_value {
        println!("value is: {}", value);
    }

    if let Some(value) = absent_string_value {
        println!("This won't print: {}", value);
    }

    println!("Success!");
}

#[test] // Тест 19
fn test19() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;

    for number in numbers.iter() {
        sum += number;
    }

    assert_eq!(sum, 15);
    println!("Success!");
}

#[test] // Тест 20
fn test20() {
    let mut counter = 0;

    let result: Vec<i32> = (1..5).map(|x| {
        counter += 1;
        x * 2
    }).collect();

    assert_eq!(counter, 4);
    assert_eq!(result, vec![2, 4, 6, 8]);

    println!("Success!");
}

#[test] // Тест 21
fn test21() {
    let text = "hello world";
    let count = text.split_whitespace().count();

    assert_eq!(count, 2);
    println!("Success!");
}

#[test] // Тест 22
fn test22() {
    let v = vec![1, 2, 3];
    let first = v.get(0).unwrap();

    assert_eq!(*first, 1);
    println!("Success!");
}

#[test] // Тест 23
fn test23() {
    let v = vec![1, 2, 3];

    let second = match v.get(1) {
        Some(&value) => value,
        None => 0,
    };

    assert_eq!(second, 2);
    println!("Success!");
}

#[test] // Тест 24
fn test24() {
    let v = vec![1, 2, 3];
    let third = v.get(2).expect("There should be three elements!");

    assert_eq!(*third, 3);
    println!("Success!");
}

#[test] // Тест 25
fn test25() {
    let mut v = vec![1, 2, 3];
    v.push(4);

    assert_eq!(v.len(), 4);
    println!("Success!");
}

#[test] // Тест 26
fn test26() {
    let mut v = vec![1, 2, 3];
    let last = v.pop();

    assert_eq!(last, Some(3));
    assert_eq!(v.len(), 2);
    println!("Success!");
}

#[test] // Тест 27
fn test27() {
    let v = vec![1, 2, 3];
    let first = &v[0];

    assert_eq!(*first, 1);
    println!("Success!");
}

#[test] // Тест 28
fn test28() {
    let mut v = vec![1, 2, 3];
    v[1] = 4;

    assert_eq!(v[1], 4);
    println!("Success!");
}

#[test] // Тест 29
fn test29() {
    let mut v = vec![1, 2, 3];
    v.sort();

    assert_eq!(v, vec![1, 2, 3]);
    println!("Success!");
}

