mod old;

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    let p1: Point<i32> = Point {x: 6, y: 29};
    println!("{:#?}", p1);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}