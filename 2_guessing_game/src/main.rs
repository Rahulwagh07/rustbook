use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess_correct_random_number();
    guess_correct_word();
    check_is_number_odd_or_even();
}

fn guess_correct_random_number() {
    println!("Welcome to guessing game...");

    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut user_guess = String::new();
        //take input
        io::stdin()
            .read_line(&mut user_guess)
            .expect("failed to read the input");

        //parse the input to num
        //shadowing.. using same var name
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {user_guess}");

        match user_guess.cmp(&secret_num) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too less"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}

fn guess_correct_word() {
    let secret_word = "rust";
    let mut guess_word = String::new();

    loop {
        println!("Enter your guess:");

        guess_word.clear();
        // println!("RAW BUFFER: {:?}", guess_word);

        io::stdin()
            .read_line(&mut guess_word)
            .expect("Failed to read input");

        let guess_word = guess_word.trim();

        println!("You guessed: {guess_word}");

        if guess_word == secret_word {
            println!("correct");
            break;
        } else {
            println!("incorrect.. try again");
        }
    }
}

fn check_is_number_odd_or_even() {
    println!("Check the num is odd or even:");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        if input % 2 == 0 {
            println!("{input} is even");
        } else {
            println!("{input} is odd");
        }

        break;
    }
}
