use std::io;
use rand::Rng;


fn main() {
    println!("Hello, world!");
   
    println!("Please enter a number:");
    
    let mut guess = String::new(); // mutable 
    //let guess = String::new();
    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
    println!("You guessed: {guess}");
}
