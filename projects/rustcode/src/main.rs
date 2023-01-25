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


}
