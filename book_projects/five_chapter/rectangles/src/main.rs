struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.area() > other.area() {
            return true
        }

        false
    }
}

fn main() {
    let rectangle = Rectangle { width: 70, height: 140 };
    let rectangle_two = Rectangle { width: 30, height: 40 };
    let rectangle_three = Rectangle { width: 80, height: 140 };

    println!("Check area two: {}", rectangle.can_hold(&rectangle_two));
    println!("Check area three: {}", rectangle.can_hold(&rectangle_three));
}

