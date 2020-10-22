// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }


// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: user1.email,
//         username: String::from("anotherusername567"),
//         active: user1.active,
//         sign_in_count: user1.sign_in_count,
//     };

//     user1.email = String::from("changed@qq.com");
    
//     println!("{}", user1.email);
//     println!("{}", user2.email);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn change_width (&mut self, new_width: u32) {
        self.width = new_width
    }
    fn create () -> Rectangle {
        Rectangle {
            width: 23,
            height: 32
        }
    }
}


fn main() {
    let mut rect1 = Rectangle::create();
    println!("rect1 is {:#?}", rect1);
    println!("rect1 area is {}", rect1.area());
    // Rectangle::change_width(&mut rect1, 32); // still valid
    rect1.change_width(32);
    println!("rect1 area after change is {}", rect1.area());
}