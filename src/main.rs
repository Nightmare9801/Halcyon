use std::{io, time::Instant};

pub mod golden_ratio;

fn main() {
    let now: Instant = Instant::now();
    {
        golden_ratio::approximate();
    }
    let elapsed: std::time::Duration = now.elapsed();
    println!("Time Taken: {:.2?}", elapsed);
    println!("Enter any character to exit");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
        }
        Err(error) => println!("error: {error}"),
    }
}
