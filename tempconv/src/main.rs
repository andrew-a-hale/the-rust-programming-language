use std::io;

enum TempUnit {
    Celius,
    Fahrenheit,
}

fn main() {
    println!("Please enter a temperature as a number: ");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
    let temp = temp.trim().parse::<i32>().expect("Please enter temperature as a number.");

    println!("Please enter temperature unit to convert to (c/f): ");

    let mut unit = String::new();
    io::stdin().read_line(&mut unit).unwrap();
    let unit = {
        let unit = unit.trim().to_lowercase();
        if unit == "c".to_string() {
            TempUnit::Celius
        } else if unit == "f".to_string() {
            TempUnit::Fahrenheit
        } else {
            panic!("Expected 'c' or 'f', got {unit}");
        }
    };

    match unit {
        TempUnit::Fahrenheit => println!("{} Celius is {} Fahrenheit", temp, temp * 9 / 5 + 32),
        TempUnit::Celius => println!("{} Fahrenheit is {} Celius", temp, (temp - 32) * 5 / 9),
    }
}
