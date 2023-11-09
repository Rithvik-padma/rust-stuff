use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=99);

    loop {
        println!("\nInput your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            },
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Small!");
                println!("Would you like to try again? (Y/N)");
            },
            Ordering::Greater => {
                println!("Large!");
                println!("Would you like to try again? (Y/N)");
            },
            Ordering::Equal => {
                println!("You got it right!");
                break;
            }
        }
        
        let mut to_continue = String::new();

        io::stdin()
            .read_line(&mut to_continue)
            .expect("Failed to read your continuation opinion!"); 
        
        if !((to_continue.trim() == "Y") || (to_continue.trim() == "y")) {
            println!("Do you want to know the number? (Y/N)");
            let mut opinion = String::new();

            io::stdin()
                .read_line(&mut opinion)
                .expect("Failed to read your opinion!");

            if (opinion.trim() == "Y") || (opinion.trim() == "y") {
                println!("The correct number is: {secret_number}");
            }
            break;
        }
    }
}