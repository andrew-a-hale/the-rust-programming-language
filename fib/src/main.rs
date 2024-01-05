use ibig::{ubig, UBig};
use std::{collections::HashMap, env, time::SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();

    let start = SystemTime::now();
    let mut cache: HashMap<usize, UBig> = HashMap::new();
    cache.insert(0, ubig!(0));
    cache.insert(1, ubig!(1));
    cache.insert(2, ubig!(1));

    let ans = fast_fib(n as usize, &mut cache);
    println!("The fib({n}) fibonacci is {ans}");
    println!("Duration: {:?}", start.elapsed().unwrap());
}

fn fast_fib(n: usize, cache: &mut HashMap<usize, UBig>) -> UBig {
    if cache.contains_key(&n) {
        return cache.get(&n).unwrap().clone();
    }

    let ans = (fast_fib((n / 2) + 1, cache) * fast_fib(n - (n / 2), cache))
        + (fast_fib(n / 2, cache) * fast_fib(n - (n / 2) - 1, cache));
    cache.insert(n, ans.clone());

    ans
}
