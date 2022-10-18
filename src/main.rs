use rand::{Rng, thread_rng};
use rand::distributions::{Alphanumeric, DistString};

mod old;

fn main() {
    let mut rng = thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);

    println!("bounded int: {}", rng.gen_range( 0.0..100.0));

    let rand_string: String = Alphanumeric.sample_string(&mut thread_rng(), 20);
    println!("{}", rand_string);
}
