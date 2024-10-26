fn draw_tree(levels: usize) {
    let trunk_height = levels;
    let trunk_width = 3;
    let max_width = 2 * levels + (levels - 1);
    for level in 0..levels {
        for row in 0..=level {
            let stars = "*".repeat(1 + 2 * row);
            let spaces = " ".repeat((max_width - stars.len()) / 2);
            println!("{}{}", spaces, stars);
        }
    }
    for _ in 0..trunk_height {
        let spaces = " ".repeat((max_width - trunk_width) / 2);
        println!("{}{}", spaces, "*".repeat(trunk_width));
    }
}
fn main() {
    let levels = 5;
    draw_tree(levels);
}
