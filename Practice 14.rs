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

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut area = 0;
    let mut overlaps = vec![];

    for i in 0..xs.len() {
        let r1 = &xs[i];
        let width1 = (r1.b.x - r1.a.x).abs();
        let height1 = (r1.b.y - r1.a.y).abs();
        area += width1 * height1;

        for j in i + 1..xs.len() {
            let r2 = &xs[j];
            if rectangles_overlap(r1, r2) {
                overlaps.push(get_overlap_area(r1, r2));
            }
        }
    }

    for overlap in overlaps {
        area -= overlap;
    }

    area
}

fn rectangles_overlap(r1: &Rectangle, r2: &Rectangle) -> bool {
    r1.a.x < r2.b.x && r1.b.x > r2.a.x && r1.a.y > r2.b.y && r1.b.y < r2.a.y
}

fn get_overlap_area(r1: &Rectangle, r2: &Rectangle) -> i32 {
    let x_overlap = (r1.b.x.min(r2.b.x) - r1.a.x.max(r2.a.x)).max(0);
    let y_overlap = (r1.a.y.min(r2.a.y) - r1.b.y.max(r2.b.y)).max(0);
    x_overlap * y_overlap
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Зайнята площа: {}", occupied);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}
