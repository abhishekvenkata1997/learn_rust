
pub fn ref_main() {
    
    // The rules of reference
    //1. At any given time, you have either one mutable reference
    //or any given number of immutable references
    //2. References must always be valid (Cannot use dangling pointers)

    //3. Understanding slices
    let mut s = String::from("Hello World");
    let word = first_word(&s); //word gets the first word, but string can be easily cleared as the word is not connected to string

    let hello = &s[0..5]; //ref to string from 0(inclusive) to 5(exclusive) or [..5]
    let world = &s[6..11]; //ref to string from 6(i) to 11(exclusive) or [6..]

    let entire_str = &s[..]; //entire string

    let word2 = first_word_slice(&s);
    println!("{}",word2);
    s.clear();
    // println!("{}", word2); cant write this because s.clear() is a mutable action(ref) on the string, 
    //and the word2 is an immutable ref, so it cant be used anymore

    let s2 = "Hello World"; //string literals are also string slices (Points to the binary)
    let word3 = first_word_slice_literal(s2); //s2 and &s2 both work
    println!("{}",word3);

    //array slice
    let a = [1,2,3,4,5];
    let slice = &a[0..2]; //array stores list of signed 32 bit integers type would be &[i32]
}

//goal is to take reference to a string and use the first word of the string
fn first_word(s: &String) -> usize {

    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; //returns point where space is found
        }
    }
    s.len() //if no space return entire length
}

//&String references to a string &str is to reference a string literal
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_literal(s: &str) -> &str {

    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
