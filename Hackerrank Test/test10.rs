use std::io;

fn grading_students(grades: Vec<i32>) -> Vec<i32> {
    grades.into_iter().map(|grade| {
        if grade >= 38 {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        } else {
            grade
        }
    }).collect()
}

fn main() {
    let mut grades: Vec<i32> = Vec::new();
    let stdin = io::stdin();

    println!("Введіть кількість оцінок:");
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let count: usize = input.trim().parse().expect("Please type a number!");

    println!("Введіть оцінки:");
    for _ in 0..count {
        let mut grade_input = String::new();
        stdin.read_line(&mut grade_input).expect("Failed to read line");
        let grade = grade_input.trim().parse::<i32>();
        if let Ok(g) = grade {
            grades.push(g);
        } else {
            println!("Неправильний формат оцінки, спробуйте ще раз.");
        }
    }

    let result = grading_students(grades);

    println!("Результати:");
    for grade in result {
        println!("{}", grade);
    }
}
