enum Shapes {
    Rectangle(u32, u32),
    Square(u32),
}

impl Shapes {
    fn area(&self) -> u32 {
        match self {
            Shapes::Rectangle(l, w) => l * w,
            Shapes::Square(s) => s * s,
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

#[derive(Debug)]
struct Square {
    side: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

#[derive(Debug)]
enum ShapeKind {
    RectKind(Rectangle),
    SqKind(Square),
}

fn main() {
    let rec = Shapes::Rectangle(3, 2);
    let sq = Shapes::Square(3);
    let shapes = [rec, sq];
    for shape in shapes.iter() {
        println!("{}", shape.area());
    }

    let rec = ShapeKind::RectKind(Rectangle {
        length: 3,
        width: 2,
    });
    let sq = ShapeKind::SqKind(Square { side: 3 });
    let shapes = [rec, sq];
    for shape in shapes.iter() {
        let area = match shape {
            ShapeKind::RectKind(rect) => rect.area(),
            ShapeKind::SqKind(sq) => sq.area(),
        };
        println!("{}", area)
    }

    let rec = ShapeKind::RectKind(Rectangle {
        length: 3,
        width: 2,
    });
    let sq = ShapeKind::SqKind(Square { side: 3 });
    let shapes = [rec, sq];
    for shape in shapes.iter() {
        if let ShapeKind::RectKind(Rectangle { length, width }) = shape {
            println!("{},{}", length, width)
        }
        if let ShapeKind::SqKind(Square { side }) = shape {
            println!("{:?}", side)
        }
    }

    let rec = ShapeKind::RectKind(Rectangle {
        length: 3,
        width: 2,
    });
    let sq = ShapeKind::SqKind(Square { side: 3 });
    let shapes = [rec, sq];
    for shape in shapes.iter() {
        if let ShapeKind::RectKind(rect) = shape {
            println!("{:?}", shape);
            println!("{}", rect.area());
        }
        if let ShapeKind::SqKind(sq) = shape {
            println!("{:?}", shape);
            println!("{}", sq.area());
        }
    }

    let rec = ShapeKind::RectKind(Rectangle {
        length: 3,
        width: 2,
    });
    let sq = ShapeKind::SqKind(Square { side: 3 });
    let shapes = [rec, sq];
    for shape in shapes.iter() {
        match shape {
            ShapeKind::RectKind(rect) => {
                println!("{:?}", shape);
                println!("{}", rect.area());
            }
            ShapeKind::SqKind(sq) => {
                println!("{:?}", shape);
                println!("{}", sq.area());
            }
        }
    }
}
