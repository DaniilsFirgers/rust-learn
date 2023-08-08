use crate::back_of_house::Appetizer;
mod front_of_house;
pub use crate::front_of_house::hosting;
// use as keyword to distinguish between the same names
use std::fmt::Result;
use std::io::Result as IoResult;

// bring multiple items into scope
use std::{cmp::Ordering, io};

// top bring in all items into scope
use std::collections::*;

// use of super keyword
fn deliver_order() {}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// Use keyword (use it within the correct module)
// use crate::front_of_house::hosting;

// now hosting is public and available to others by callin restaurant::hosting::add_to_waitlist()
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change our mind on bread
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);
    // Absolute path (the function is defined in the same crate, thus we can use 'crate' keyword)
    hosting::add_to_waitlist();

    // Relative path
    // hosting::add_to_waitlist();

    // use of enums

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Use keyword
