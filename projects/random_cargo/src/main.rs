use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

  loop{
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut sec_s = String::new();
    sec_s = secret_number.to_string();
    
    println!("\n========= keep this secret ==========\nthe string is : {sec_s}");

    println!("Hello, world!");
   
    println!("Please enter a number:");
    
    let mut guess = String::new(); // mutable 
    //let guess = String::new();
    io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
    println!("You guessed: {guess}");
    println!("The secret number is {secret_number}");

    //let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let guess: u32 = match guess.trim().parse() 
               {
                  Ok(num) => num,
                  Err(_) => continue,
               };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => 
            {
                println!("You win!");
                break;
            }
    }
  }

  let a = [3; 5];
  println!("the array is {},{},{},{},{}",a[0],a[1],a[2],a[3],a[4]);
  let mut a: [i32; 5] = [1, 2, 3, 4, 5]; 
  
  print_array(&mut a);

  print_array(&mut a);

  let result = array_plus1(&mut a);
  println!("the result is {result}");

  let mut aLEN = i32::from(a.len() as i32);
  println!("the length is {aLEN}");

  for i in 0..aLEN
  {
     println!("i is {i}");
  }

  // now we will call a normal print_array 
  normal_print_array(a);
  let mut aLEN2 = i32::from(a.len() as i32);  
  for i in 0..aLEN2
  {
     println!("i is {i}");
  }

  let s = String::from("hello");  // s comes into scope
  takes_ownership(s); 
  //println!("string is still here? {s}"); this didn't work any more
  let s = String::from("hello");  // s comes into scope
  takes_ownershipv2(&s);
  println!("string is still here? {s}");

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn takes_ownershipv2(some_string: &String) { // some_string comes into scope
    println!("{}", some_string);
}

fn normal_print_array(x:[i32;5])
{
   let aLEN = x.len(); 
   println!("the length is ... {}",aLEN);
}


fn print_array(x:&mut [i32]) {
    let y2 = x;
    for (i, item) in y2.iter().enumerate()
    {
       println!("i is {i}");
       println!("item is {item}");
    }

    9;
    /*println!("The value of array is: {:?}",x);
    x[0] = 100;


    for item in x
    {
       println!("Element: {item}");
    }
    */
    //let x2 = &mut x;    // can we do that?  no, you cannot, the x is moved 
    //let aLEN  = y2.len(); 
    //for i in x.iter_mut()
    //{println!("i ii {i}");}
    //let x2 = x.clone(); 
    //let mut aLEN = i32::from(x2.len() as i32);
    //let arrayLEN:&usize = &x.len();
    //println!("the length is {}",aLEN);
    //for i in 0..aLEN
    //{
    //   println!("loop on ele: {}",i);
    //}
}

fn array_plus1(x:&mut [i32])->i32 
{
   x[0]+=1;
   return x[0];
}


