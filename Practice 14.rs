#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(rectangles: &[Rectangle]) -> i32 {
    let mut total_area = 0;

    for i in 0..rectangles.len() {
        let width = (rectangles[i].b.x - rectangles[i].a.x).abs();
        let height = (rectangles[i].b.y - rectangles[i].a.y).abs();
        total_area += width * height;

        for j in i + 1..rectangles.len() {
            if rectangles_overlap(&rectangles[i], &rectangles[j]) {
                total_area -= overlap_area(&rectangles[i], &rectangles[j]);
            }
        }
    }

    total_area
}

fn rectangles_overlap(r1: &Rectangle, r2: &Rectangle) -> bool {
    r1.a.x < r2.b.x && r1.b.x > r2.a.x && r1.a.y > r2.b.y && r1.b.y < r2.a.y
}

fn overlap_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = (r1.b.x.min(r2.b.x) - r1.a.x.max(r2.a.x)).max(0);
    let y_overlap = (r1.a.y.min(r2.a.y) - r1.b.y.max(r2.b.y)).max(0);
    x_overlap * y_overlap
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle { a: Point { x: 2, y: 9 }, b: Point { x: 5, y: 3 } },
        Rectangle { a: Point { x: 1, y: 8 }, b: Point { x: 11, y: 6 } },
        Rectangle { a: Point { x: 9, y: 10 }, b: Point { x: 13, y: 2 } },
    ]
}

fn main() {
    let occupied = area_occupied(&test_data());
    println!("Occupied area: {}", occupied);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let occupied = area_occupied(&test_data());
        assert_eq!(occupied, 60);
    }
}

