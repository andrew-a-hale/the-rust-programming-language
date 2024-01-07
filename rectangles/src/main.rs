#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }
}

fn main() {
    let rec1 = Rectangle { length: 30, width: 50 };
    dbg!(&rec1);
    println!("{}", rec1.area());

    let rec2 = Rectangle { length: 40, width: 50 };
    let rec3 = Rectangle { length: 30, width: 60 };
    let rec4 = Rectangle { length: 20, width: 50 };
    let rec5 = Rectangle { length: 30, width: 40 };
    
    let recs = [rec2, rec3, rec4, rec5];
    for rec in recs.iter() {
        println!("{}", rec1.can_hold(&rec))
    }

    let sq = Rectangle::square(10);
    println!("{}", sq.area());

    println!("{}", Rectangle::square(100).can_hold(&recs[0]));
    println!("{}", Rectangle::square(10).can_hold(&recs[0]));
}
