use guitar;

fn main() {
    let strings = guitar::create_strings(6, vec![6]);
    let neck = guitar::create_neck(50, strings);
    let body = guitar::create_body(neck);
    let guitar = guitar::Guitar::new(body);

    println!("{:?}", guitar)
}
