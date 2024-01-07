use rug::{Integer, ops::{MulFrom, AddFrom}};
use std::{env, time::SystemTime, ops::AddAssign};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();
    let start = SystemTime::now();
    let mut ans = calc(n as usize);
    println!("The fib({n}) fibonacci is {}", &ans);
    println!("Duration: {:?}", start.elapsed().unwrap());
    assert!(is_fib(&mut ans));
}

// handle fib(2n) = fib(n) * (2 * fib(n + 1) - fib(n))
// and fib(2n + 1) = fib(n + 1) ^ 2 + fib(n) ^ 2
fn calc(n: usize) -> Integer {
    let (a, b) = fib(n / 2);
    if n % 2 == 0 {
        Integer::from(&a * (Integer::from(2) * b - &a))
    } else {
        a.square() + b.square()
    }
}

// fib(n), fib(n + 1)
// using fib(2n) and fib(2n + 1) identites above
fn fib(n: usize) -> (Integer, Integer) {
    if n == 0 {
        (Integer::from(0), Integer::from(1))
    } else {
        let (a, b) = fib(n / 2);
        let mut new_a = Integer::from(&a * (Integer::from(2) * &b - &a));
        let new_b =  a.square() + b.square();
        if n % 2 == 0 {
            // returns fib(n), fib(n + 1)
            (new_a, new_b)
        } else {
            // returns fib(n + 1), fib(n) + fib(n + 1)
            new_a.add_assign(&new_b);
            (new_b, new_a)
        }
    }
}

// checking if fib(n) is valid by property
// fib(n) is true <=> 5 * n^2 +/- 4 is a perfect square
fn is_fib(n: &mut Integer) -> bool {
    // calc 5 * n * n
    n.square_mut();
    n.mul_from(5);

    // check +4
    n.add_from(4);
    if n.is_perfect_square() {
        return true
    }

    // check -4
    n.add_from(-8);
    if n.is_perfect_square() {
       return true 
    } 

    false
}
