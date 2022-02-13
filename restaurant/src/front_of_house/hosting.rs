pub use crate::back_of_house::cooking::{Appetizer, Breakfast};

pub fn eat_at_restaurant(apt: Appetizer) {
    seat_at_table();
    crate::front_of_house::hosting::add_to_waitlist();
    super::hosting::add_to_waitlist();
    let mut breakfast = Breakfast::summer("Rye");
    breakfast.toast = String::from("Wheat");
    println!("I'd like {} toast please", breakfast.toast);
    let order = apt;

}

pub fn add_to_waitlist() { }

fn seat_at_table() { }