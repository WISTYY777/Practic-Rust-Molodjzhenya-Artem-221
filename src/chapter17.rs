#[test] // Тест 1
fn test1() {
    let i = 3;                                             
    {                                                    
        let borrow1 = &i; 
        println!("borrow1: {}", borrow1);
    } 
    {                                                    
        let borrow2 = &i; 
        println!("borrow2: {}", borrow2);               
    }                                                   
}

#[test] // Тест 2
fn test2() {
    let x = 5;            
    let r = &x;           
    println!("r: {}", r); 
}                         

#[test] // Тест 3
fn test3<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test] // Тест 4
fn test4() {
    let invalid_output = || -> &'static String { 
        &String::from("foo") 
    };
}

#[test] // Тест 5
fn test5<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

#[test] // Тест 6
fn test6() {
    let _x = 12;
    let y: &'static i32 = &_x; 
}

#[test] // Тест 7
fn test7() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
}

#[test] // Тест 8
fn test8() {
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);
    
    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

#[test] // Тест 9
fn test9() {
    let var_a = 35;
    let example: Example;
    
    {
        let var_b = NoCopyType {};
        example = Example { a: &var_a, b: &var_b };
    }
    
    println!("(Success!) {:?}", example);
}

#[test] // Тест 10
fn test10() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!");
}

#[test] // Тест 11
fn test11() {
    let part = "Some part";
    let excerpt = ImportantExcerpt { part };
    excerpt.level();
}

#[test] // Тест 12
fn test12<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

#[test] // Тест 13
fn test13<'a>(x: &'a i32) -> &'a i32 { x }

#[test] // Тест 14
fn test14<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

#[test] // Тест 15
fn test15() {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
}

#[test] // Тест 16
fn test16() {
    let age = 30;
    let name = "John Doe";
    let person = Person { age, name };
}

#[test] // Тест 17
fn test17() {
    let i = 5;
    need_static(i);
}

#[test] // Тест 18
fn test18() {
    unsafe {
        config = init();
        println!("{:?}", config);
    }
}

#[test] // Тест 19
fn test19() {
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);
}

#[test] // Тест 20
fn test20() {
    static NUM: i32 = 18;
    let static_ref = coerce_static(&NUM);
}

#[test] // Тест 21
fn test21<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

#[test] // Тест 22
fn test22() {
    let mut string = "First".to_owned();
    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
}
