use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("");
    println!("Welcome to the Roulette Guessing Game!");
    println!("Les jeaux sont fait");

    loop {
        let secret_number = rand::thread_rng().gen_range(0, 36);

        println!("");
        println!("");
        println!("Place your bet on a number between 0 and 36");
      
        let mut bet = String::new();
            io::stdin()
                .read_line(&mut bet)
                .expect("Failed to read line");

        let bet:u32 = match bet.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if bet > 36 {
            println!("Invalid input. Please enter a number between 0 and 36:");
            continue;
        }

        println!("Place your bet on black or red:");

        let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
        
        let guess = guess.trim().to_lowercase();
        
        println!("You guessed: {}", bet);
        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);
    
        match bet.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Sorry, the number is too small and you lost. Better luck next time!".red()),
            Ordering::Greater => println!("{}", "Sorry, the number is too big and you lost. Better luck next time!".red()),
            Ordering::Equal => {
                println!("{}", "Congratulation, you won!".green());
                break;
            }
        }
        
        if guess == "black" && is_black(secret_number) {
            println!("{}", "Congratulations! You guessed the right color!".green());
        } else if guess == "red" && is_red(secret_number) {
            println!("{}", "Congratulations! You guessed the right color!".green());
        } else {
            println!("{}", "Sorry, you didn't guessed the right color. Better luck next time!".red());
        }
    }
}

fn is_black(number: u32) -> bool {
    let black_numbers: [u32; 18] = [
        2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35,
    ];
    black_numbers.contains(&number)
}

fn is_red(number: u32) -> bool {
    let red_numbers: [u32; 18] = [
        1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
    ];
    red_numbers.contains(&number)
}
