const WIDTH: usize = 21;  
const HEIGHT: usize = 13; 

fn main() {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 {
                print!("*");
            } else if j == 0 || j == WIDTH - 1 {
                print!("*");
            } else if j == i && i < HEIGHT / 2 {
                print!("*");
            } else if j == WIDTH - 1 - i && i < HEIGHT / 2 {
                print!("*");
            } else if j == (i - HEIGHT / 2 + HEIGHT / 2) && i >= HEIGHT / 2 {
                print!("*");
            } else if j == (WIDTH - 1 - (i - HEIGHT / 2)) && i >= HEIGHT / 2 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
