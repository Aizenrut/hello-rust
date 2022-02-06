fn main() {
    fibonacci(8);
}

fn fibonacci(n: u32) {
    println!("First {} fibonacci numbers:", n);
    if n == 0 { return };

    let mut penultimate = 1;
    println!("{}", penultimate);
    if n == 1 { return };

    let mut ultimate = 1;
    println!("{}", ultimate);
    if n == 2 { return };

    let mut next: u32;

    for _ in 0..n-2 {
        next = ultimate + penultimate;
        penultimate = ultimate;
        ultimate = next;

        println!("{}", next);
    }
}