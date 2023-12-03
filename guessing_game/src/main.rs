use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=99);

    'guessNumber: loop {
        println!("\nInput your guess:");

        let mut guess: String = String::new();
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
        
        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Small!");
            },
            Ordering::Greater => {
                println!("Large!");
            },
            Ordering::Equal => {
                println!("You got it right!");
                break;
            }
        }

        'toContinue: loop {
            println!("\nDo you want to try again? (Y/N)");

            let mut to_continue: String = String::new();

            io::stdin()
            .read_line(&mut to_continue)
            .expect("Failed to read your continuation opinion!");
        
            if !((to_continue.trim() == "Y") || (to_continue.trim() == "y")) {
                if !((to_continue.trim() == "N") || (to_continue.trim() == "n")) {
                    println!("Invalid input, please try again!");
                    continue 'toContinue;
                }

                println!("Do you want to know the number? (Y/N)");
                let mut opinion: String = String::new();

                io::stdin()
                    .read_line(&mut opinion)
                    .expect("Failed to read your opinion!");

                if (opinion.trim() == "Y") || (opinion.trim() == "y") {
                    println!("The correct number is: {secret_number}");
                }
                else {
                    println!("Thank you!");
                }
                break 'guessNumber;
            }
        }
 
        
    }
}