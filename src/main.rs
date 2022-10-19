use crate::Suit::{Club, Diamond, Heart, Spade};

mod old;

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]

fn main() {
    let p1 = if 2 > 1 { true } else { false };
    println!("{:#?}", p1);
    print_choice(Heart);
    print_choice(Spade);
    print_choice(Club);
    print_choice(Diamond);
    country(32);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
    println!("Oranges are {}", get_oranges(11));

    let point = (0, 0);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("{} and {}", x, y)
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "1 or 2",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount",
        _ => "lots of"
    }
}



enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => { println!("\u{2665}") }
        Spade => { println!("\u{2660}") }
        Club => { println!("\u{2663}") }
        Diamond => { println!("\u{2666}") }
    }
}