use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let xs = vec![3, 4, 11, 12, 6, 1, 2, 3, 5, 7, 4];
    println!("Median: {}", median(&xs));
    println!("Mode: {:?}", mode(&xs));
}

fn median(xs: &Vec<i32>) -> f32 {
    let mut sorted = xs.clone();
    sorted.sort();

    let len = xs.len();
    let midpoint = len / 2;
    if len % 2 == 0 {
        (sorted[midpoint - 1] + sorted[midpoint]) as f32 / 2.0
    } else {
        sorted[midpoint] as f32
    }
}

fn mode(xs: &[i32]) -> Vec<i32> {
    let mut digits = HashMap::new();

    for v in xs.iter() {
        let count = digits.entry(*v).or_insert(0);
        *count += 1
    }

    let mut highest = 0;
    let mut mode = Vec::new();
    for (k, v) in digits {
        match v.cmp(&highest) {
            Ordering::Equal => mode.push(k),
            Ordering::Greater => {
                highest = v;
                mode = vec![k];
            }
            Ordering::Less => (),
        };
    }

    mode
}
