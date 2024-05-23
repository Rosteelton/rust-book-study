use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    println!("Secret number = {secret_number}");
    println!("-----------------");

    loop {
        println!("Guess the number");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
