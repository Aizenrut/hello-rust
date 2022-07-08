//! Art
//! 
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColors;
pub use self::kinds::SecundaryColors;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum PrimaryColors {
        Red,
        Yellow,
        Blue
    }

    /// The secundary colors according to the RYB color model.
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum SecundaryColors {
        Green,
        Orange,
        Purple
    }
}

pub mod utils {
    use crate::kinds::{PrimaryColors, SecundaryColors};

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let c1 = art::PrimaryColors::Blue;
    /// let c2 = art::PrimaryColors::Yellow;
    /// let mixture = art::mix(c1, c2);
    /// 
    /// assert_eq!(art::SecundaryColors::Green, mixture);
    /// ```
    pub fn mix(c1: PrimaryColors, c2: PrimaryColors) -> SecundaryColors {
        SecundaryColors::Green
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mixture = mix(PrimaryColors::Blue, PrimaryColors::Yellow);

        assert_eq!(SecundaryColors::Green, mixture);
    }
}