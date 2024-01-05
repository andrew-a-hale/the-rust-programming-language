use rug::Integer;
use std::{collections::HashMap, env, time::SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();

    let start = SystemTime::now();
    let mut cache: HashMap<usize, Integer> = HashMap::new();
    cache.insert(0, Integer::from(0));
    cache.insert(1, Integer::from(1));
    cache.insert(2, Integer::from(1));

    let ans = calc(n as usize);
    println!("The fib({n}) fibonacci is {}", ans);
    println!("Duration: {:?}", start.elapsed().unwrap());
}

// handle fib(2n) = fib(n) * (2 * fib(n + 1) - fib(n))
// and fib(2n + 1) = fib(n + 1) ^ 2 + fib(n) ^ 2
fn calc(n: usize) -> Integer {
    let (a, b) = fib(n / 2);
    if n % 2 == 0 {
        Integer::from(&a * (Integer::from(2) * b - &a))
    } else {
        Integer::from(&a * &a) + Integer::from(&b * &b)
    }
}

// fib(n), fib(n + 1)
fn fib(n: usize) -> (Integer, Integer) {
    if n == 0 {
        (Integer::from(0), Integer::from(1))
    } else {
        let (a, b) = fib(n / 2);
        let new_a = Integer::from(&a * (Integer::from(2) * &b - &a));
        let new_b = Integer::from(&a * &a) + Integer::from(&b * &b);
        if n % 2 == 0 {
            // returns fib(n), fib(n + 1)
            (new_a, new_b)
        } else {
            let new_c = Integer::from(&new_a + &new_b);
            // returns fib(n + 1), fib(n) + fib(n + 1)
            (new_b, new_c)
        }
    }
}
