use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;  

fn main() {
    let a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1,2,3, 4, 5]; //implicit type cast  to Vec<i32>
    //let third  = &v[2]; //this is a run-time error not a compile time error (This is also an immutable borrow )
    let third = v[2];
    v.push(6); //this is a mutable reference 
    //for arrays we know size at compile time, vectors are on heap not on stack meaning it will crash at run-time not compile time
    //safer way to access these vectors using get method
    println!("The third element of the vector is {}",third);

    match v.get(2) {
        Some(third) => println!("The third element using match is {}",third),
        None => println!("There is no third element!")
    }
    //v.get function returns an option using match expression to match and check for either of these cases

    // vectors will be dropped when it goes out of scope

    let mut v = vec![1,2,3,4,5];

    for i in &mut v {
        println!("{}",i);
        *i += 50;
    }

    for i in &v {
        println!("{}",i);
    }

    //vectors store data of only one type, why not make that type an enum to store multiple formats inside my vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Second element")),
        SpreadsheetCell::Float(14.32)
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}",i),
        _ => println!("Not an integer")
    };


    //Lower programming languages need to deal with complexity of a string
    //Collection of UTF-8 encoded bytes

    let s1 = String::new();
    let s2 = "intial contents"; //string literals(slices)
    let s3 = s2.to_string(); //convert slice to string
    let s4 = String::from("initial contents");

    let mut s = String::from("Hello ");
    s.push_str("World");
    s.push('!');
    println!("{}",s);
    let s5 = String::from("foo");
    let s6 = String::from(" bar");
    let s7 = s5 + &s6;
    let s8 = format!("{} {}",s7,s6);
    println!("{}",s8);
    println!("{}",s7);

    let hello: String = String::from("Hello");
    //let c: char  = hello[0];
    //println!("{}",c);

    for b in hello.bytes() {
        println!("{}",b);
    }

    for c in hello.chars() {
        println!("{}",c);
    }

    for g in hello.graphemes(true) {
        println!("{}",g);
    }

    //hashmaps
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    //passing blue and yellow pushes ownership of them into our scores hashmap 
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);



    //let mut scores = HashMap::new();

    scores.insert(String::from("Green"), 30);
    scores.insert(String::from("Green"),45); //overwrite key with new value

    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Green")).or_insert(50);

    for(key, value) in &scores {
        println!("{} : {}",key,value);
    } 

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let cnt = map.entry(word).or_insert(0);
        *cnt += 1
    }
    println!("{:#?}",map);
}   
