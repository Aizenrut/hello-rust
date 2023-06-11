use std::collections::HashMap;
use std::io;

fn main() {
    println!("Add <employee> to <department>");

    let mut map = HashMap::new();

    loop {
        println!();

        let mut input = String::new();

        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => break,
                Err(_) => {
                    println!("Invalid input");
                    continue;
                }
            };
        }

        let message = input.trim();

        if message.eq("done") {
            break;
        }

        let add = "Add ";
        let add_index: usize;

        match message.find(add) {
            Some(index) => add_index = index,
            None => {
                println!("Your input does not contain 'Add'");
                continue;
            }
        };

        let to = " to ";
        let to_index: usize;

        match message.find(to) {
            Some(index) => to_index = index,
            None => {
                println!("Your input does not contain 'to'");
                continue;
            }
        };

        let name = message[add_index + add.len()..to_index].trim();
        let department = message[to_index + to.len()..].trim();

        let employees = map.entry(department.to_string()).or_insert(vec![]);
        employees.push(name.to_string());

        println!("Added {} to {}", name, department);
    }

    println!();

    for (dep, employees) in &mut map {
        println!("{}:", dep);

        employees.sort();

        for name in employees {
            println!("\t{}", name);
        }
    }
}
