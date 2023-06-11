fn add_3(value: i32) -> i32 {
    value + 3
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Value must be greater or equal than 1");
        } else if value > 100 {
            panic!("Value must be smaller or equal than 100");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_adds_3() {
        let addition = add_3(3);

        assert_eq!(
            6, addition,
            "Addition of 3 + 3 didn't result in 6. Result: {}",
            addition
        );
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 20,
            height: 10,
        };

        let smaller = Rectangle {
            width: 10,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 20,
            height: 10,
        };

        let smaller = Rectangle {
            width: 10,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected = "Value must be greater or equal than 1")]
    fn guess_cannot_be_smaller_than_1() {
        Guess::new(-1);
    }

    #[test]
    #[should_panic(expected = "Value must be smaller")]
    fn guess_cannot_be_greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}
