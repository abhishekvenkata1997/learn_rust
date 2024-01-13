pub fn flows() {

    let number = 34;

    if number < 10 {
        println!("Condition 1 is true");
    } else if number < 22 {
        println!("Second condition is true");
    } else {
        println!("Condition was false");
    }

    let condition = true;
    let num_check = if condition {5} else {6};
    println!("Checking for number: {}",num_check);

    //looping

    //1. Normal loop (Here we can assign values to a loop and also get a return value from the loop)
    let mut counter = 0;

    let result = loop{
        counter+=1;
        //println!("again");

        if counter == 10 {
            break counter;
        }
    };
    println!("Here is the result for my loop! = {}",result);

    //2. While loop
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    //3. For n loop

    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("The value is {}",element);
    }

    for number in 1..4 {
        println!("{}!",number);
    }
    
    println!("All loops are dead");
}