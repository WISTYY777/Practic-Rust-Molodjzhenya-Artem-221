#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[test] // Тест 1
fn test_1_rectangle_area() {
    let rect1 = Rectangle { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }

    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

#[test] // Тест 2
fn test_2_traffic_light_show_state() {
    let light = TrafficLight {
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);
}

#[test] // Тест 3
fn test_3_traffic_light_change_state() {
    let mut light = TrafficLight {
        color: "red".to_owned(),
    };
    light.change_state();
    assert_eq!(light.color, "green");
}

#[test] // Тест 4
fn test_4_traffic_light_new() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
}

#[test] // Тест 5
fn test_5_rectangle_can_hold() {
    let rect1 = Rectangle { width: 10, height: 10 };
    let rect2 = Rectangle { width: 5, height: 5 };
    assert!(rect1.can_hold(&rect2));
}

#[test] // Тест 6
fn test_6_rectangle_can_hold_false() {
    let rect1 = Rectangle { width: 5, height: 5 };
    let rect2 = Rectangle { width: 10, height: 10 };
    assert!(!rect1.can_hold(&rect2));
}
