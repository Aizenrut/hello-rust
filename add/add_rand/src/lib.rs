use rand::Rng;

pub fn add_rand(x: i32) -> i32 {
    x + rand::thread_rng().gen::<i8>() as i32
}