#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// you can include many methods and associated funcs in one impl block or create many impl blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // youi can use Self or name of Enum (in this case Rectangle)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // ASSOCIATED FUNCTIONS

    let sq = Rectangle::square(3);
    println!("Square is {:#?}", sq);
}
