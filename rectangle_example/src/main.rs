#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 100,
        height: 150,
    };

    println!("The area of the rectangle is: {}", calculate_area(&rect));
    println!("With #[derive(Debug)] we can see the rect: {:?}", rect);
    println!("The area of the rectangle is: {}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(rect3));

    let square = Rectangle::square(15);
    println!("Did a square with associated function: {:#?}", square);
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
