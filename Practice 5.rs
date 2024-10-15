const WIDTH: usize = 28;  
const HEIGHT: usize = 13; 

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 {
                output.push('*');
            } else if j == 0 || j == WIDTH - 1 {
                output.push('*');
            } else if j >= i && j < WIDTH - i {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    println!("{}", output);
}
