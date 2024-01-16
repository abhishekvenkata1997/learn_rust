//when to use enums
// example ip address only has ipv4 and ipv6
//only two types using enums makes sense

enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr{
    kind: IpAddrKind,
    address: String
}
enum IpAddrKind2 {
    V4(String), //can store data inside a variable name in enum 
    V6(String)
}

enum IpAddrKind3 {
    V4(u8,u8,u8,u8), //can store complex value inside v4
    V6(String)
}

enum Message {
    Quit, //no data
    Move {x: i32, y: i32}, //store anon struct
    Write(String), //store a single string
    ChangeColor(i32,i32,i32) //store 3 integers
}

//Just like structs enums have methods and assoc functions
impl Message {
    fn some_function() {
        println!("Lets learn rust");
    }
}



//one way to store IP Addresses
fn main() {
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost  = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let localhost2  = IpAddrKind2::V4(String::from("127.0.0.0.1"));

    let localhost3 = IpAddrKind3::V4(127,0,0,1);

    //Option enum
    //Many languages have null values, value can exist or it can be null
    //Type system cant gaurantee if you use a value its not null
    //RUST HAS NO NULL VALUES
    /*enum Option<T> { // T can be any type generic value
        Some(T),
        None
    }*/
    //If something can be null it has to be wrapped inside option enum
    //Option enum exists as a pre-existing scope inside like i32, f64, String and so on

    let some_number = Some(5);
    let some_string = Some("Hi Abhishek here");

    let absent_number: Option<i32> = None; //explicitly declare type

    let x: i8 = 5;
    let y: Option<i8> = None;

    // let sum = x+y; wont work because we cant add an option to a normal number
    let sum2 = x+y.unwrap_or(x); //if y is missing it uses the default value

    
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none_val = plus_one(None);
    let seven = plus_one_all(six);
    check_cond();
    //println!("Value of five six and none are {} {} {}", five, six, none_val);
}

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        Arizona,
        Arkansas,
        California,
        //
    }
    enum Coin {
        Penny,
        Nickel,
        Dime, 
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }

        }
    }
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    //conditions where there are several cases and we dont want to handle all of them
    fn plus_one_all(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i+1),
            _ => None
        }
    }

    fn check_cond() {
        //better to use match index 
        let some_value = Some(3);
        if let Some(3) = some_value {
            println!("Three");
        }
    }


fn route(ip_kind: IpAddrKind) {

}