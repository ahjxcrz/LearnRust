use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

  loop{
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut sec_s = String::new();
    sec_s = secret_number.to_string();
    println!("the string is : {sec_s}");

    println!("Hello, world!");
   
    println!("Please enter a number:");
    
    let mut guess = String::new(); // mutable 
    //let guess = String::new();
    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
    println!("You guessed: {guess}");
    println!("The secret number is {secret_number}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
  }
}
