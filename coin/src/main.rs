use rand::{self, Rng};

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewYork,
    Utah,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    for _ in 0..10 {
        let random_coin = rand::thread_rng().gen_range(0..5);

        let coin = match random_coin {
            0 => Some(Coin::Penny),
            1 => Some(Coin::Nickel),
            2 => Some(Coin::Dime),
            3 => {
                let random_state = rand::thread_rng().gen_range(0..5);

                let state = match random_state {
                    0 => Some(UsState::Alabama),
                    1 => Some(UsState::Alaska),
                    2 => Some(UsState::California),
                    3 => Some(UsState::NewYork),
                    4 => Some(UsState::Utah),
                    _ => None,
                };

                let c = if let Some(s) = state {
                    Some(Coin::Quarter(s))
                } else {
                    None
                };

                c
            }
            _ => None,
        };

        match coin {
            Some(Coin::Quarter(state)) => println!("State quarter from {:?}", state),
            Some(c) => println!("{:?}", c),
            None => println!("None"),
        };
    }
}
