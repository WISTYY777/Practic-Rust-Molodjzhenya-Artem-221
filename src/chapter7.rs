// Тест 1
#[test]
fn test1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

// Тест 2
#[test]
fn test2() {
    let n = 5;

    let big_n =
        if n < 10 && n > -10 {
            println!("{}, and is a small number, increase ten-fold", n);
            10 * n
        } else {
            println!("{}, and is a big number, halve the number", n);
            n / 2
        };

    println!("{} -> {}", n, big_n);
}

// Тест 3
#[test]
fn test3() {
    for n in 1..=99 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}

// Тест 4
#[test]
fn test4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in numbers {
    }

    println!("{:?}", numbers);
}

// Тест 5
#[test]
fn test5() {
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

// Тест 6
#[test]
fn test6() {
    let mut n = 1;

    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

// Тест 7
#[test]
fn test7() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

// Тест 8
#[test]
fn test8() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}

// Тест 9
#[test]
fn test9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

// Тест 10
#[test]
fn test10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    println!("Success!");
}

// Тест 11
#[test]
fn test11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}

