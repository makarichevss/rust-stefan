use std::io::stdin;

#[allow(dead_code)]
fn input_output() {
    //! # Main function
    //!reads user input from
    //! ```
    //! fn main()
    //! ```
    println!("Hello, world!");
    let mut input = String::new();
    println!("Say something");

    match stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    }
}
