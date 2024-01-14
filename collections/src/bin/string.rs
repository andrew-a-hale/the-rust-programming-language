fn main() {
    println!("apple => {}", to_pig_latin("apple").unwrap());
    println!("egg => {}", to_pig_latin("egg").unwrap());
    println!("first => {}", to_pig_latin("first").unwrap());
    println!("=> {}", to_pig_latin("").unwrap());
}

#[derive(Debug)]
enum Error {
    EmptyString,
}

fn to_pig_latin(s: &str) -> Result<String, Error> {
    match s.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => Ok(format!("{s}-hay")),
        Some(x) => Ok(format!("{}-{x}ay", &s[1..])),
        None => Err(Error::EmptyString),
    }
}
