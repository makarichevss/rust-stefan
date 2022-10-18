mod old;

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    let mut primes: Vec<i32> = Vec::new();
    primes.push(12);
    let mut primes2 = [2, 3, 5, 7, 11];
    println!("{:?}", primes);
    println!("{:?}", primes2);

    let mut vect = vec![1; 10];
    let slice = &mut primes2[1..3];
    println!("{:?}", slice[1]);
    println!("{:?}", slice);
    let mut colors = ["red", "yellow", "blue"];
    colors[0] = "red";
    update_colors(&mut colors[1..3]);
    println!("{:?}", colors);

    let person = ("John", 27, true);
    let (a, b, _) = person;
    println!("{:?}", b);
}

fn update_colors(color_slice: &mut [&str]) {
    color_slice[0] = "red";
}
