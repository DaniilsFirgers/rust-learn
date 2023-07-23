//constant
const MY_NAME: &str = "Daniils";

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // this is okay
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces length {spaces}");

    // this will not work (cannot change typre of mut) :(
    // let mut test = "  "
    // let test = test.len()
}
