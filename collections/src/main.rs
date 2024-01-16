

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
    ]

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}",i),
        _ => println!("Not an integer")
    };
}
