pub fn main2() {
    
    // _Ownership Rules_
    // 1. Each value in Rust has a variable thats called its owner
    // 2. There can be only one owner at a time
    // 3. When owner goes out of scope, the value is dropped

    {
        // s is not valid here
        let s = "Hello";// s is valid here but immutable
        let s = String::from("Hello"); //memory allocated to heap, when goes out of scope auto removed
        // do stuff with s

    } // the scope is over and value is dropped


    let x = 5;
    let y = x; //Copy [This works for chars, ints and smaller generic types]

    let s1 = String::from("Hello");
    //let s2 = s1; // Move (Not just point to s1)
    //dont do direct copy
    let s2 = s1.clone(); //need to do clone to make sucha  copy
    println!("The value of string is {}",s1); //cant do s2 = s1, this will give ownership issues
}


//new function taking ownership
pub fn main3() {
    let s = String::from("Hello");
    takes_ownership(s);
    //println!("{}",s); wont work because ownership changes for s, s is moved to new variable

    let x = 5;
    makes_copy(x);
    println!("{}",x); //this makes a copy

    //giving ownership to a new variable
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back_ownership(s2);
    println!("s1 = {} s3 = {}",s1,s3);
}

fn makes_copy(some_val: i32) {
    println!("{}",some_val);
}
fn takes_ownership(some_string: String) {
    println!("{}",some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string //takes ownership from some_string and gives it to s1
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string //takes ownership from s2 to a_string and then gives it back to s3
}

pub fn main4() {
    let s1 = String::from("Hello");
    //let (s2, len) = calculate_length(s1); [Function Takes ownership and returns length, ownership goes to s2 ]
    let (len) = calculate_length_ref(&s1); //[Function passes ref, no ownership lost]
    println!("The length of '{}' is {}.",s1, len);
    let mut s3 = String::from("Hello"); //need to write a mutable variable if you want it updated
    change_ref(&mut s3); //Passing a mutable reference to the function that is borrowing the string
    println!("s3 is mutable new value is {}",s3);

    //limitation of mutable references
    let mut s4 = String::from("This is a mutable string we want to write references for and borrow them");
    let r1 = &mut s4;
    //let r2 = &mut s4; second mutable borrow is illegal
    //let r2 = &s4; cannot borrow variable as immutable when it is already borrowd once as a mutable reference
    //This helps prevent data races at compile time -> Occures when two pointers are pointed to same data, and one is mutable, so we get corrupt data
    println!("One mutable references {}",r1);
    let mut s5 = String::from("Mutable string");
    let r1 = &s5;
    let r2 = &s5;
    println!("Two immutable references to s5 are -> {} , {}",r1,r2); //scope of a reference is from when its first used to when its last used

    let r3 = &mut s5;
    println!("One mutable reference to s5 {}",r3); //r1 and r2 are out of scope here

    //let ref_to_nothing = dangle(); dangling pointer causes compile time error
}   

fn calculate_length(s: String) -> (String, usize) {
    
    let length = s.len();//len() returns length of the string
    (s, length)
}

fn calculate_length_ref(s: &String) -> (usize) {
    //s.push_str("Hello"); [Borrowed variables are not mutable]
    let length = s.len();
    length
}

fn change_ref(some_string: &mut String) {
    some_string.push_str(" World");
}

/*fn dangle() -> &String {
    let s = String::from("Hello");
    return &s;
    //&s //s goes out of scope when value is returned, so &s points to nothing, this gives a compile time error, 
    //because s is expired but pointer of s is returned
}
*/ //wont work because its a dangling pointer