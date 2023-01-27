//#[derive(Debug)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


use crate::garden::vegetables::Asparagus;

pub mod garden;


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}

fn main() {

   let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
   
   println!("user name: {}", user1.username);  

   let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
   println!("rect1 is {:#?}", rect1);

   let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

   println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
   
   let rect3 = Rectangle::square(10);
   println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


   let plant = Asparagus {}; 
   println!("I'm growing {:?}!", plant);

}
