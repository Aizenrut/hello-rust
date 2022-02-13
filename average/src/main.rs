use std::collections::HashMap;

fn main() {
    let mut v = vec![0, 1, 2, 3, 1, 2, 4, 5, 6, 7, 9, 8, 2, 1, 3, 6, 4];
    println!("vec {:?}", v);

    let median = median(&mut v);
    println!("median: {}", median);

    let mode = mode(&v);
    println!("mode: {:?}", mode);
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();

    let middle = v.len() / 2;

    v[middle]
}

fn mode(v: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = vec![];

    for k in map.keys() {
        if let Some(count) = map.get(k) {
            if count > &max {
                mode = vec![*k];
                max = *count;
            } else if count == &max {
                mode.push(*k);
            }
        }
    }

    mode
}