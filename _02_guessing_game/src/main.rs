use random_number::random;
use std::io;

fn main() {
    println!("Guess the number");
    println!("================");
    println!("Please input your guess.");

    let n: u32 = random!(1..10);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Oops some thing went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number guess");
                continue;
            }
        };

        match guess.cmp(&n) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
