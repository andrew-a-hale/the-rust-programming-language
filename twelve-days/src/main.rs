use std::collections::HashMap;

fn main() {
    let lines = HashMap::from([
        (12, "drummers drumming,"),
        (11, "pipers piping,"),
        (10, "lords a-leaping,"),
        (9, "ladies dancing,"),
        (8, "maids a-milking,"),
        (7, "swans a-swimming,"),
        (6, "geese a-laying,"),
        (5, "golden rings,"),
        (4, "calling birds,"),
        (3, "French hens,"),
        (2, "turtle doves,"),
        (1, "a partridge in a pear tree!"),
    ]);

    for day in 1..=12 {
        println!("On the {} day of Christmas,\nmy true love gave to me", n_to_ordinal(day));

        for line in (2..=day).rev() {
            println!("{} {}", n_to_number(line), lines.get(&line).unwrap())
        }

        if day > 1 {
            println!("And {}\n", lines.get(&1).unwrap())
        } else {
            println!("{}\n", lines.get(&1).unwrap())
        }
    }
}

fn n_to_ordinal(n: i32) -> String {
    match n {
        1 => "First",
        2 => "Second",
        3 => "Third",
        4 => "Fourth",
        5 => "Fifth",
        6 => "Sixth",
        7 => "Seventh",
        8 => "Eigth",
        9 => "Nineth",
        10 => "Tenth",
        11 => "Eleventh",
        12 => "Twelveth",
        _ => panic!("Invalid day")
    }.to_string()
}

fn n_to_number(n: i32) -> String {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        _ => panic!("Invalid day")
    }.to_string()
}