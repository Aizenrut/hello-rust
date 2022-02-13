fn main() {
    let mut v = vec![0, 1, 2, 3];

    for i in &v {
        println!("{}", i);
    }

    println!();

    v.push(4);
    v.push(5);
    v.push(6);

    for i in &mut v {
        *i += 3;
        println!("{}", i);
    }

    println!();

    let mut v = Vec::new();
    v.push(10);

    println!("len: {}", v.len());
    println!("pop: {}", v.pop().unwrap());
    println!("len: {}", v.len());

    println!();

    let v = Vec::from([9, 8, 7, 6]);

    for i in &v {
        println!("{}", i);
    }
}
