use std::io;

fn main() {

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    println!("Enter a sentence: ");
    let mut sentence = String::new();
    
    loop {
        match io::stdin().read_line(&mut sentence) {
            Ok(_) => break,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
    }

    let mut words: Vec<&str> = sentence.trim().split(' ').collect();
    let mut result = String::new();

    for word in &mut words {
        let mut chars = word.chars();
        let first_char = chars.next();

        match first_char {
            Some(c) => {
                if vowels.contains(&c) {
                    result.push_str(&format!("{}-hay ", word)[..]);
                } else {
                    let ord = chars.as_str();
                    result.push_str(&format!("{}-{}ay ", ord, c)[..]);
                }
            },
            None => continue
        }
    }

    println!("Pig-latin:");
    println!("{}", result);
}
