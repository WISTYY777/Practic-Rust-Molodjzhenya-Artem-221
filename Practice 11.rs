fn is_palindrome(num: i32) -> bool {
    let original = num;
    let mut reversed = 0;
    let mut n = num;

    while n != 0 {
        let digit = n % 10;
        reversed = reversed * 10 + digit;
        n /= 10;
    }

    original == reversed
}

fn main() {
    let test_numbers = vec![121, -121, 10, 12321];

    for &num in &test_numbers {
        if is_palindrome(num) {
            println!("{} є паліндромом.", num);
        } else {
            println!("{} не є паліндромом.", num);
        }
    }
}
