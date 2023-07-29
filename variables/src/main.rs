//constant
const MY_NAME: &str = "Daniils";

fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {x}");

    println!("The value of x is: {x}");

    // this is okay
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces length {spaces}");

    // this will not work (cannot change typre of mut) :(
    // let mut test = "  "
    // let test = test.len()

    // signed (can take neagtive and positive values)/unsigned (can only be positive) integers
    let random_int: u16 = 12345;

    // floats can be f32 and f64
    let random_float: f32 = 123.45;

    // compound types

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // access it like this or
    let five_hundred = tup.0;
    println!("The value of y is: {y}");

    // array (it should have a fixed length)
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3;  5] == let a = [3,3,3,3,3]
    // accessing array elements
    let first = array[0];
}
