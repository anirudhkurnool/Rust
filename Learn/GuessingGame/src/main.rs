use colored::*;
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("========= Guessing Game =========");
    

    //generating random secret answer
    let mut rng = rand::rng();
    let secret_answer : u8 = rng.random_range(..100);
    println!("secret answer = {}\n", secret_answer);

    //taking in the user input and checking it against the answer
    loop {
        let mut user_guess = String::new();//an empty string
        println!("Enter your guess : ");
        io::stdin().read_line(&mut user_guess).expect("failed to read user input");
        println!("user input : {}", user_guess);
        let user_guess:u8= match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //catch all errors
                println!("please enter numbers between 0 and 100\n");
                continue;
            }
        };

        match user_guess.cmp(&secret_answer) {
            Ordering::Less=>println!("{}\n","less than the answer".red()),
            Ordering::Greater=>println!("{}\n", "greater than the answer".red()),
            Ordering::Equal=>{
                println!("{}\n", "equal to the answer".green());
                break;
            }
        }
    }
    
}
