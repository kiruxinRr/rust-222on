#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(rect1.area(), 1500);
    }
}

// TrafficLight структура та її методи
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("the current state is {}", self.color);
    }
}

#[cfg(test)]
mod traffic_light_tests {
    use super::*;

    #[test]
    fn test_traffic_light() {
        let light = TrafficLight {
            color: "red".to_owned(),
        };
        light.show_state();
        println!("{:?}", light);
    }
}

// TrafficLight з методами `show_state` та `change_state`
impl TrafficLight {
    pub fn show_state(self: &Self) {
        println!("the current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}

#[cfg(test)]
mod traffic_light_change_tests {
    use super::*;

    #[test]
    fn test_traffic_light_change_state() {
        let mut light = TrafficLight {
            color: "red".to_owned(),
        };
        light.change_state();
        assert_eq!(light.color, "green");
    }
}

// TrafficLight з асоційованою функцією `new` та методом `get_state`
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: "red".to_string(),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

#[cfg(test)]
mod traffic_light_new_tests {
    use super::*;

    #[test]
    fn test_traffic_light_new() {
        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");
    }
}

// Rectangle з додатковим методом `can_hold`
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_can_hold_tests {
    use super::*;

    #[test]
    fn test_rectangle_can_hold() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 20, height: 40 };
        assert!(rect1.can_hold(&rect2));
    }
}

// TrafficLightColor перерахунок з методом `color`
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    fn color(&self) -> String {
        match *self {
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green => "green".to_string(),
        }
    }
}

#[cfg(test)]
mod traffic_light_color_tests {
    use super::*;

    #[test]
    fn test_traffic_light_color() {
        let c = TrafficLightColor::Yellow;
        assert_eq!(c.color(), "yellow");
    }
}

fn main() {
    println!("Run `cargo test` to execute the tests.");
}
