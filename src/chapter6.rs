// Тест 1
#[test]
fn test1() {
    let s: &str = "hello, world";

    println!("Success!");
}

// Тест 2
#[test]
fn test2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s);
}

fn greetings(s: &str) {
    println!("{}", s);
}

// Тест 3
#[test]
fn test3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

// Тест 4
#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}

// Тест 5
#[test]
fn test5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

// Тест 6
#[test]
fn test6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

// Тест 7
#[test]
fn test7() {
    let s = String::from("hello, world");
    greetings(&s);
}


// Тест 8
#[test]
fn test8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}

// Тест 9
#[test]
fn test9() {
    let byte_escape = "I'm writing Ru\x73__!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

    let long_string = "String literals can span multiple lines. The linebreak and indentation here can be escaped too!";
    println!("{}", long_string);
}


// Тест 11
#[test]
fn test11() {
    let bytestring: &[u8; 21] = b"this is a byte string";

    println!("A byte string: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, like with normal raw strings"#;

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb";

    match std::str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}

// Тест 12
#[test]
fn test_12() {
    let s = String::from("hi,中国");

    let chars: Vec<char> = s.chars().collect();
    assert_eq!(chars[0], 'h');
    assert_eq!(chars[1], 'i');
    assert_eq!(chars[2], ',');
    assert_eq!(chars[3], '中');
    assert_eq!(chars[4], '国');
}

// Тест 13
#[test]
fn test13() {
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

// Тест 14
#[test]
fn test14() {
    fn init_arr(n: usize) -> [i32; 100] {
        [1; 100]
    }

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5);

    println!("Success!");
}

// Тест 15
#[test]
fn test_15() {
    let arr = [1, 2, 3];
    assert_eq!(std::mem::size_of_val(&arr), 12);
}


// Тест 16
#[test]
fn test16() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

// Тест 17
#[test]
fn test17() {
    let _arr = [1, 2, 3];

    println!("Success!");
}

// Тест 18
#[test]
fn test18() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[1];

    assert!(ele == 'b');

    println!("Success!");
}

// Тест 19
#[test]
fn test19() {
    let names = [String::from("Sunfei"), String::from("Sunface")];

    let name0 = names.get(0).unwrap();

    let name1 = &names[1];

    println!("Success!");
}

// Тест 20
#[test]
fn test20() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("Success!");
}

// Тест 21
#[test]
fn test21() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}

// Тест 22
#[test]
fn test22() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

// Тест 23
#[test]
fn test23() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[0..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}

// Тест 24
#[test]
fn test_24() {
    let s = "你好，世界";
    let substring: String = s.chars().take(2).collect();
    assert_eq!(substring, "你好");
    println!("Success!");
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}

// Тест 26
#[test]
fn test26() {
    let _t0: (u8,i16) = (0, -1);
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

// Тест 27
#[test]
fn test27() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "am");

    println!("Success!");
}

// Тест 28
#[test]
fn test28() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple length: {}", std::mem::size_of_val(&too_long_tuple)); // Виправлено
}

// Тест 29
#[test]
fn test29() {
    let tup = (1, "hello", 4.5);
    let (x, y, z) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 4.5);
}

// Тест 30
#[test]
fn test30() {
    let mut counter = 0;

    loop {
        if counter == 10 {
            break;
        }
        counter += 1;
    }

    assert_eq!(counter, 10);
}

// Тест 31
#[test]
fn test31() {
    let mut sum = 0;
    let mut n = 1;

    while n <= 10 {
        sum += n;
        n += 1;
    }

    assert_eq!(sum, 55);
}

// Тест 32
#[test]
fn test32() {
    for i in 1..=10 {
        println!("{}", i);
    }

    assert!(true);
}

// Тест 33
#[test]
fn test33() {
    let arr = [1, 2, 3];
    for elem in arr.iter() {
        println!("{}", elem);
    }

    assert!(true);
}

// Тест 34
#[test]
fn test34() {
    let arr = [1, 2, 3];
    let sum: i32 = arr.iter().sum();

    assert_eq!(sum, 6);
}

// Тест 35
#[test]
fn test35() {
    let sum = (1..=10).sum::<i32>();
    assert_eq!(sum, 55);
}

// Тест 36
#[test]
fn test36() {
    let s = "hello, world";
    let length = s.len();

    assert_eq!(length, 12);
}

// Тест 37
#[test]
fn test37() {
    let s = "hello, world";
    let first_char = s.chars().next().unwrap();

    assert_eq!(first_char, 'h');
}

// Тест 38
#[test]
fn test38() {
    let s = "hello, world";
    let last_char = s.chars().last().unwrap();

    assert_eq!(last_char, 'd');
}

// Тест 39
#[test]
fn test39() {
    let s = "hello, world";
    let is_empty = s.is_empty();

    assert_eq!(is_empty, false);
}

// Тест 40
#[test]
fn test40() {
    let mut s = String::from("hello");
    s.push_str(", world");

    assert_eq!(s, "hello, world");
}

// Тест 41
#[test]
fn test41() {
    let s = String::from("hello");
    let len = s.len();

    assert_eq!(len, 5);
}

// Тест 42
#[test]
fn test42() {
    let s = String::from("hello");
    let char_at_index = s.chars().nth(1).unwrap();

    assert_eq!(char_at_index, 'e');
}

// Тест 43
#[test]
fn test43() {
    let s = String::from("hello");
    let slice = &s[0..2];

    assert_eq!(slice, "he");
}

// Тест 44
#[test]
fn test44() {
    let s = String::from("hello, world");
    let words: Vec<&str> = s.split_whitespace().collect();

    assert_eq!(words, vec!["hello,", "world"]);
}

// Тест 45
#[test]
fn test45() {
    let s = String::from("hello");
    let replaced = s.replace("e", "a");

    assert_eq!(replaced, "hallo");
}
