use rug::Integer;
use std::{env, time::SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();
    let start = SystemTime::now();
    let ans = calc(n as usize);
    // println!("The fib({n}) fibonacci is {}", &ans);
    assert!(is_fib(&ans));
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
// using fib(2n) and fib(2n + 1) identites above
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

// checking if fib(n) is valid by property
// fib(n) is true <=> 5 * n^2 +/- 4 is a perfect square
fn is_fib(n: &Integer) -> bool {
    let s = 5 * Integer::from(n * n);
    is_square(Integer::from(4 + &s)) || is_square(Integer::from(-4 + &s))
}

fn is_square(n: Integer) -> bool {
    let s = Integer::sqrt(n.clone());
    Integer::from(&s * &s) == Integer::from(n)
}
