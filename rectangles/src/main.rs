#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // rectangle using tuples
    let rect_tuple = (30, 50);
    println!(
        "The areas of rectangle is {} square pixels!",
        area_tuple(rect_tuple)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // to print struct
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
