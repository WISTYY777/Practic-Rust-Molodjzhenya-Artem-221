use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[test] // тест 1
fn test1() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

#[test] // тест 2
fn test2() {
    struct Meters(u32);

    #[test] // тест 3
    fn test3() {
        let i: u32 = 2;
        assert_eq!(i.pow(2), 4);

        let n = Meters(i);
        assert_eq!(n.0.pow(2), 4);
    }
}

#[test] // тест 4
fn test4() {
    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days));
}

#[test] // тест 5
fn test5() {
    use std::ops::Add;
    use std::fmt;

    struct Meters(u32);
    impl fmt::Display for Meters {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "There are still {} meters left", self.0)
        }
    }

    impl Add for Meters {
        type Output = Self;

        fn add(self, other: Meters) -> Self {
            Self(self.0 + other.0)
        }
    }

    fn calculate_distance(m1: Meters, m2: Meters) -> Meters {
        m1 + m2
    }

    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}", d), "There are still 30 meters left");
}

#[test] // тест 6
fn test6() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    fn operate(op: VeryVerboseEnumOfThingsToDoWithNumbers, x: i32, y: i32) -> i32 {
        match op {
            VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
            VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
        }
    }

    let x = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
}

#[test] // тест 7
fn test7() {
    fn my_function(n: usize) -> [u32; 10] {
        [123; 10]
    }

    let arr = my_function(10);
    println!("{:?}", arr);
}

#[test] // тест 8
fn test8() {
    let s: &str = "Hello there!";
    let arr: [u8; 3] = [1, 2, 3];
}

#[test] // тест 9
fn test9() {
    use std::fmt::Display;

    fn foobar(thing: impl Display) {}

    fn main() {}
}
