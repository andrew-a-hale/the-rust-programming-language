use std::{env, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u64 = args[1].parse().unwrap();
    let suffix = suffix(n);
    let ans = fib(n);
    println!("The {n}{suffix} fibonacci is {ans}");
}

fn fib(n: u64) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();
    cache.insert(0, 1);
    cache.insert(1, 1);

    fn _fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if cache.contains_key(&n) {
            *cache.get(&n).unwrap()
        } else {
            let ans = _fib(n - 1, cache) + _fib(n - 2, cache);
            cache.insert(n, ans);
            ans

        }
    }

    _fib(n, &mut cache)
}

fn suffix(n: u64) -> String {
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