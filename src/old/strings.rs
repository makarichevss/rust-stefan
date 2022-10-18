const URL: &str = "google.com";

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn string_of_text() {
    let name = "Sergey";
    let cat_name: &'static str = "Barsik";
    println!("{}", cat_name);

    let cat = String::new();
    let mut cat2 = String::from(cat_name);
    println!("{}", cat2);

    cat2.push_str(" is nice");
    let owner = format!("Hi, I'm the owner of {}", cat2);
    println!("{} {}", owner, URL);
}
