mod map;

fn main() {
    println!("apple => {}", to_pig_latin("apple"));
    println!("first => {}", to_pig_latin("first"));
}

fn to_pig_latin(s: &str) -> String {
    let first = match s.chars().next() {
        Some(x) => x,
        None => panic!("Error: empty string"),
    };

    match first {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{s}-hay"),
        _ => format!("{}-{first}ay", &s[1..]),
    }
}
