use std::io;

fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) {
    let anna_share = (bill.iter().sum::<i32>() - bill[k]) / 2;
    let result = if b == anna_share {
        String::from("Bon Appetit")
    } else {
        (b - anna_share).to_string()
    };
    println!("{}", result);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (_, k): (usize, usize) = {
        let mut iter = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };
    
    let mut bill = String::new();
    io::stdin().read_line(&mut bill).unwrap();
    let bill: Vec<i32> = bill.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut b_temp = String::new();
    io::stdin().read_line(&mut b_temp).unwrap();
    let b = b_temp.trim().parse::<i32>().unwrap();

    bon_appetit(bill, k, b);
}
