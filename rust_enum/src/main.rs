// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// impl IpAddr {
//     fn say (&self) {
//         println!("{:#?}", &self);
//     }
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
//     home.say();
//     loopback.say();
// }

// fn main () {
//     let num_0 = 5;
//     let num_1 = Option::Some(5);
//     let num_2 = Option::Some(10);
//     let str_1 = Option::Some("a string");
//     let absent_number: Option<i32> = Option::None;
    
//     println!("{}", num_1.unwrap() + num_0);
//     println!("{}", num_1.unwrap() + num_2.unwrap());
//     println!("{}", num_1.unwrap() + absent_number.unwrap());
// }

// fn main () {
//     #[derive(Debug)] // so we can inspect the state in a minute
//     enum UsState {
//         Alabama,
//         Alaska,
//         // --snip--
//     }

//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }
    
//     fn value_in_cents(coin: Coin) -> u8 {
//         match coin {
//             Coin::Penny => 1,
//             Coin::Nickel => 5,
//             Coin::Dime => 10,
//             Coin::Quarter(state) => {
//                 match state {
//                     Alabama => 200,
//                     Alaska => 30
//                 }
//             },
//         }
//     }
//     let coin_1 = Coin::Dime;
//     let coin_2 = Coin::Quarter(UsState::Alabama);

//     println!("{}", value_in_cents(coin_1)); // 10
//     println!("{}", value_in_cents(coin_2)); // 200
// }

fn main () {
    let num_0 = 5;
    let num_1 = Option::Some(5);
    let num_2 = Option::Some(10);
    let str_1 = Option::Some("a string");
    let absent_number: Option<i32> = Option::None;
    
    fn plus_one (num: Option<i32>) -> Option<i32> {
        match num {
            Some (v) => Some(v + 1),
            None => None,
        }
    }
    println!("{:#?}", plus_one(num_1));
    println!("{:#?}", plus_one(absent_number));
}