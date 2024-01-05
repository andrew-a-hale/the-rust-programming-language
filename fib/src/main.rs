use std::env;
use ibig::{UBig, ubig};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();

    let suffix = suffix(n);
    let ans = fib(n);
    println!("The {n}{suffix} fibonacci is {ans}");
}

fn fib(n: u32) -> UBig {
    let mut a = ubig!(1);
    let mut b = ubig!(1);
    let mut c: UBig;
    if n > 1 {
        for _ in 1..n {
            c = b.clone();
            b = b + a;
            a = c;
        }
    }

    b
}

fn suffix(n: u32) -> String {
    let n = n % 10;
    if (4..9).contains(&n) || n == 0 {
        "th".to_string()
    } else if n == 1 {
        "st".to_string()
    } else if n == 2 {
        "nd".to_string()
    } else if n == 3 {
        "rd".to_string()
    } else {
        panic!("unknown suffix");
    }
}