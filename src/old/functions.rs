#[allow(unused_variables)]
#[allow(dead_code)]

pub fn functions() {
    let greeting = format!("Hello there");
    for i in 1..6 {
        println!("{}", greeting);
    }
    let mut name = "Vasya";
    say_hello(&mut name);
}

fn say_hello(name: &mut &str) {
    println!("Hello {}", name);
    *name = "Sergey";
    println!("Hello {}", name);
}