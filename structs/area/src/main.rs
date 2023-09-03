#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn create_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scalar = 5;
    let rect1 = Rectangle {
        width: dbg!(scalar * 30),
        height: 50,
    };

    if rect1.width() {
        println!("rect1 is {:#?}", rect1);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle { width: 10, ..rect1 };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let square = Rectangle::create_square(10);

    println!("square is {:#?}", square);
}
