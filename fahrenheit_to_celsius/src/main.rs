fn main() {
    println!("Fahrenheit to Celsius:");

    let fahrenheit: f64 = 1.0;
    println!("Fahrenheit: {}", fahrenheit);

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("Celsius: {}", celsius);
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32f64) / 1.8
}
