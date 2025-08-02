struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rectangle = Rectangle { width: 70, height: 140 };

    println!("Area: {}", area(&rectangle));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
