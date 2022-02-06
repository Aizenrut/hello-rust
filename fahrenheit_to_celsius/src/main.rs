use std::io;

fn main() {
    loop {
        println!("Fahrenheit: ");
        let mut fahrenheit = String::new();

        match io::stdin()
            .read_line(&mut fahrenheit) {
            Ok(_) => (),
            Err(_) => {
                println!("Could not read the input");
                continue;
            }
        }

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        let celsius = fahrenheit_to_celsius(fahrenheit);

        println!("Celsius: {}", celsius);
        break;
    }

}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32f64) / 1.8
}
