#[derive(Debug)]
struct Rectangle {
    height: u32,
    width:  u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    println!("Area: {}", area_v1(30, 50));
    let dimensions = (30, 50);
    println!("Area: {}", area_v2(dimensions));
    let rectangle = Rectangle {
        height: 30,
        width: 50
    };

    let other_rect = Rectangle {
        height: 10,
        width: 20
    };
    println!("Area: {}", area_v3(&rectangle));
    println!("Rectangle is {:?}", rectangle);
    println!("Area: {}", rectangle.area());
    println!("Can Hold? {}", rectangle.can_hold(&other_rect));
    println!("Can Hold? {}", other_rect.can_hold(&rectangle));

    let square = Rectangle::square(3);
    println!("Area: {}", square.area());
}

fn area_v1(height: u32, width: u32) -> u32 {
    height * width
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
