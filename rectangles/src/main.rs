#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let sq = Rectangle::square(30);

    let can_hold = rect1.can_hold(&sq);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("The react can hold sq? {can_hold}");
}
