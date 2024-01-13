
mod final_file;
use final_file::main2;
mod control_flows;
use control_flows::flows;
fn main() {
    
    let x = 5;
    println!("The value of x is {}",x);

    let x = "six"; //variable shadowing
    println!("The value of x is {}",x);

    const SUB_COUNT : i64= 100_000;

    //Integers
    let a = 300; //default is 32 bit
    let a:u8 = 255;//max limit is 255 for signed 8 bit variables

    //Floating point
    let b = 32.2; //default is f64
    let c : f32 = 21.1; //can declare different size as well 

    //boolean
    let _d =  true;
    let _e : bool = false; //not necessary to declare variables (if declared rust asks to use underscore)
    
    //Character
    let f = 'z';
    let _g: char = 'Z';

    //Compound Types
    let tup  = ("I am abhishek",26);//fixed size array of related data which can be of different type
    let (name,age) = tup;
    let my_age = tup.1;

    let array_codes = [200,321,145,3400];
    let not_found = array_codes[3];
    let x = array_codes[0];
    let byte = [0;8]; //8 elements all with value zero
    main2();
    flows();
}
