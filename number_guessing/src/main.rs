use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("welcome to number guessing game");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Somthing wromg");

        println!("Your guess is {}", inp);

        let inp: u32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match inp.cmp(&secret) {
            Ordering::Less => println!("Val less"),
            Ordering::Greater => println!("Val greater"),
            Ordering::Equal => {
                println!("found match");
                break;
            }
        }
    }
}
