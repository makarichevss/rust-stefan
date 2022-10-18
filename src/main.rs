use crate::Colors2::Name;

mod old;

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    let my_color = Colors::Red;
    let my_color2 = Name(String::from("Vasya"));
    println!("{:#?}", my_color2);
}

#[allow(dead_code)]
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue
}

#[allow(dead_code)]
#[derive(Debug)]
enum Colors2 {
    Name(String),
    Age(i32)
}
