use add_one;
use add_rand;

fn main() {
    let x = 3;
    let plus_one = add_one::add_one(x); 
    let plus_rand = add_rand::add_rand(x); 

    println!("Add workspace");
    println!("{} plus one is {}", x, plus_one);
    println!("{} plus rand is {}", x, plus_rand);
}