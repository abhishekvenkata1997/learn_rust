use std::cmp::Ordering;
use std::io;
use rand::Rng;
use colored::*;

fn main() {

    let secret_num = rand::thread_rng().gen_range(1..100);
    println!("Number you guessed is secret_num{}",secret_num);
    loop {
        println!("Please input your guess!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("The secret num is {}",secret_num);
        
        match guess.cmp(&secret_num){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Equal => {
                println!("{}","Perfect guess!".green());
                break;
            },
            Ordering::Greater => println!("{}","Too big!".red())
        }
    }
}
