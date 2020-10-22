// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     const Y: f32 = 2.43;
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", Y);    
// }
// fn main () {
//     let x = 
//     if 5 > 3 { 
//         3
//     } else { 
//         6
//     };
//     println!("The value of x is: {}", x);

// }

// fn main () {
//     let mut counter = 0;

//     let result = loop {
//         if counter == 2 {
//             counter += 2;
//             continue;
//         }
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
//     println!("----------------------");

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }


// fn main() {
//     let range = 1..5;
//     let range = range.rev();
//     for number in range {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");
// }

// fn main () {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }

// fn main() {
//     let s = String::from("hello");
//     let s = transfer_ownership(s);
//   	println!("{}", s); // fine
//     let x = 5;                    
//     makes_copy(x);             
//   	println!("{}", x); // fine
// } 
// fn transfer_ownership(some_string: String) -> String {
//     println!("{}", some_string);
//     return some_string
// } 
// fn makes_copy(some_integer: i32) { 
//     println!("{}", some_integer);
// }


// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
//   // moved, so nothing happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("hello"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // takes_and_gives_back will take a String and return one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//     let s = String::from("hello");
//     print_string(&s);
//   	println!("{}", s); // fine
//     let x = 5;                    
//     makes_copy(x);             
//   	println!("{}", x); // fine
// } 
// fn print_string(some_string: &String) {
//     println!("{}", some_string);
// } 

// fn makes_copy(some_integer: i32) { 
//     println!("{}", some_integer);
// }

// fn main() {
//     let mut s = String::from("hello");
//   	println!("original string: {}", s); // fine
//     print_string(&mut s); // 传入引用，而不是move
//     {
//         let r1 = &mut s;
//         r1.push_str(", im visible ddd");
//     }
//     let d = &mut s;
//     print_string(d);
//   	println!("after change in print string: {}", s); // fine
//     let x = 5;                    
//     makes_copy(x);             
//   	println!("{}", x); // fine
// } 
// fn print_string(some_string: &mut String) { // 接受一个&String类型的string
//     some_string.push_str(", world");
//     println!("{}", some_string);
// } 

// fn makes_copy(some_integer: i32) { 
//     println!("{}", some_integer);
// }

// fn main () {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time

//     println!("{}, {}", r1, r2);
// }

// fn main () {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // r1 and r2 are no longer used after this point
    
//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut target = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && target == 0 {
            target = target + 1;
        }

        if item == b' ' && target == 1 {
            return &s[i+1..]
        }
    }

    &s[..]
}

fn main () {
    let s = String::from("hello world");
    let first:&str = first_word(&s);
    let second:&str = second_word(&s);
    println!("{}{}", first, second);
}