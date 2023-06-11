fn main() {
    let largest_char = largest(&vec!['a', 'z', 'x', 'u', 'v', 'w']);
    println!("The largest char is: '{}'", largest_char);

    let largest_i32 = largest(&vec![10, 75, 46, 8, 1, 42]);
    println!("The largest i32 is: {}", largest_i32);

    let largest_f32 = largest(&vec![4.2, 0.3, 8.9, 11.01, 20.20, 2.0]);
    println!("The largest f32 is: {}", largest_f32);

    let strs = vec!["adn", "fkogh", "rthq", "rjtrh", "e", "io"];
    let largest_str = largest_ref(&strs);
    println!("The largest referenced str is: \"{}\"", largest_str);

    let sizes: Vec<usize> = vec![1, 4, 2, 5, 7];
    let largest_usize = largest_ref(&sizes);
    println!("The largest referenced usize is: {}", largest_usize);
}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut largest = items[0];

    for &item in items {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_ref<T: PartialOrd>(items: &[T]) -> &T {
    let mut largest = &items[0];

    for item in items {
        if item > largest {
            largest = item;
        }
    }

    largest
}
