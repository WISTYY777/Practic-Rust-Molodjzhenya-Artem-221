#[test] // Тест 1
fn test1() {
    let mut s: String = "hello, ".to_string();
    s.push_str("world");
    s.push('!');

    move_ownership(s);

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s);
}

#[test] // Тест 2
fn test2() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s;
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s.clone();
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!");
}

#[test] // Тест 3
fn test3() {
    let s: String = String::from("hello, world!");

    let slice: &str = &s;

    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

#[test] // Тест 4
fn test4() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");

    let slice2 = &s[3..6];
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success!");
}

#[test] // Тест 5
fn test5() {
    let mut s = String::new();
    s.push('H');

    let v = vec![104, 101, 108, 108, 111];

    let s1 = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("Success!");
}

#[test] // Тест 6
fn test6() {
    let mut s = String::new();

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!");
}

use std::mem;

#[test] // Тест 7
fn test7() {
    let story = String::from("Rust By Practice");

    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!");
}

#[test] // Тест 8
fn test8() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    let v = vec!(1, 2, 3);
    is_vec(&v);

    let mut v1 = Vec::new();
    for &item in &arr {
        v1.push(item);
    }
    is_vec(&v1);

    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}

#[test] // Тест 9
fn test9() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    assert_eq!(v1, v2);

    println!("Success!");
}

#[test] // Тест 10
fn test10() {
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = arr.to_vec();

    assert_eq!(v1, v2);

    let s = "hello".to_string();
    let v1: Vec<u8> = s.into_bytes();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    let s = "hello";
    let v3 = Vec::from(s);
    assert_eq!(v2, v3);

    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}

#[test] // Тест 11
fn test11() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v[i]);
    }

    for i in 0..5 {
        v.push(i + 2);
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}

#[test] // Тест 12
fn test12() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    let slice2 = &v[0..3];

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..3];
    slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}

#[derive(Debug, PartialEq)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

#[test] // Тест 13
fn test13() {
    let v: Vec<IpAddrEnum> = vec![IpAddrEnum::V4("127.0.0.1".to_string()), IpAddrEnum::V6("::1".to_string())];

    assert_eq!(v[0], IpAddrEnum::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddrEnum::V6("::1".to_string()));

    println!("Success!");
}

trait IpAddrTrait {
    fn display(&self);
}

struct V4(String);
impl IpAddrTrait for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddrTrait for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

#[test] // Тест 14
fn test14() {
    let v: Vec<Box<dyn IpAddrTrait>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

use std::collections::HashMap;

#[test] // Тест 15
fn test15() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, &95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in &scores {
        println!("The score of {} is {}", name, score);
    }
}

#[test] // Тест 16
fn test16() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    let teams_map2: HashMap<_, _> = teams.iter().cloned().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("Success!");
}

#[test] // Тест 17
fn test17() {
    let fruits = vec!["apple", "banana", "orange"];
    let fruit_map: HashMap<_, _> = fruits.iter().map(|&fruit| (fruit, fruit.len())).collect();

    assert_eq!(fruit_map.get("banana"), Some(&6));

    println!("Success!");
}

#[test] // Тест 18
fn test18() {
    let points = vec![(1, 2), (3, 4), (5, 6)];
    let point_map: HashMap<_, _> = points.iter().map(|&(x, y)| (x, y)).collect();

    assert_eq!(point_map.get(&3), Some(&4));

    println!("Success!");
}

#[test] // Тест 19
fn test19() {
    let numbers = vec![1, 2, 3];
    let sum: i32 = numbers.iter().sum();

    assert_eq!(sum, 6);

    println!("Success!");
}

#[test] // Тест 20
fn test20() {
    let numbers = vec![1, 2, 3];
    let product: i32 = numbers.iter().product();

    assert_eq!(product, 6);

    println!("Success!");
}
