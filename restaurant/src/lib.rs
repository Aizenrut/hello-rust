mod front_of_house;
mod back_of_house;

pub use front_of_house::hosting::{self, Appetizer};

fn receive_costumer() {
    hosting::eat_at_restaurant(Appetizer::Salad);
}

fn serve_order() { }