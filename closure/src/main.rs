mod algorithms;

use algorithms::caching::Cacher;
use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 1;
    let random_number = 6;

    generate_workout(intensity, random_number);
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|x| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        x
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
