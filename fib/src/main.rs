use std::env;
use ibig::{UBig, ubig};

struct Queue {
    vals: Vec<UBig>,
}

impl Queue {
    fn new() -> Queue {
        Queue {vals: vec![ubig!(1), ubig!(1)]}
    }

    fn push(&mut self, v: &UBig) {
        self.vals.remove(0);
        self.vals.push(v.clone());
    }

    fn sumpush(&mut self) -> UBig {
        let mut s = ubig!(0);
        for v in &self.vals {
            s += v
        }
        self.push(&s);
        s
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();

    let mut cache = Queue::new();
    let suffix = suffix(n);
    let ans = fib(n, &mut cache);
    println!("The {n}{suffix} fibonacci is {ans}");
}

fn fib(n: u32, cache: &mut Queue) -> UBig {
    let mut ans = ubig!(1);
    if n > 1 {
        for _ in 1..n {
            ans = cache.sumpush();
        }
    }

    ans
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