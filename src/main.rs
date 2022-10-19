mod old;

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "hamster", "bear"];
    for pet in pets {
        if pet == "cat" {
            println!("{} meywoos", pet);
        }
        println!("I love my {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        
    }

    get_squares(10);
    get_cubes(100);
}

fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} * {0} = {1}", x, x * x);
        x += 1;
    }
}

fn get_cubes(limit: i32) {
    let mut x = 1;
    loop {
        println!("{0} * {0} = {1}", x, x * x * x);
        x += 1;
        if x * x * x > limit {
            break
        }
    }
}
