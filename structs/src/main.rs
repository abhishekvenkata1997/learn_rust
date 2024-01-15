//enums and structs building blocks used to build new types in rust
//Ch 5. Grouping related data using structs
// Define methods and associative functions     
//compare structs to tuples

mod rect;
use rect::main2;
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
//naming struct and giving names to each variable in struct
//can refer with names instead of index location

fn main() {
    
    let mut user1 = User{
        username: String::from("Abhishek"),
        email: String::from("abhishek@gmail.com"),
        sign_in_count: 4,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("Abhishek Venkata"); 

    let user2  = build_user(String::from("sreya@gmail.com"),String::from("Sreya Vallabhaneni"));
    println!("{}",user2.email); //cant print whole user only one can be printed at each time

    let user3 = User {
        email: String::from("pranav@gmail.com"),
        username: String::from("Pranav Kashyap"),
        ..user2
    }; //use user2 values for the remaining fields

    //main2();
    //create structs without name fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    //two structs have some types of variables but if a function expects a color we need to send a struct of type color, and not point
    main2();
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
} 