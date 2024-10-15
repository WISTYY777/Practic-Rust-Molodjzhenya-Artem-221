fn draw_tree(triangles: usize) {
    let trunk_height = triangles;
    let trunk_width = 3;
    let max_width = 2 * triangles + (triangles - 1);

    for t in 0..triangles {
        for level in 0..=t {
            let width = 1 + 2 * level;
            let spaces = " ".repeat((max_width - width) / 2);
            let stars = "*".repeat(width);
            println!("{}{}", spaces, stars);
        }
    }

    for _ in 0..trunk_height {
        let spaces = " ".repeat((max_width - trunk_width) / 2);
        println!("{}{}", spaces, "*".repeat(trunk_width));
    }
}

fn main() {
    let triangles = 5;
    draw_tree(triangles);
}
