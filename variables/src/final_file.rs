pub fn main2() {
    let res = my_fun(32,43);
    println!("My result is {}",res);
}

fn my_fun(x:i32, y:i32) -> i32 {
    println!("Another function x= {}",x);
    let sum = x+y;
    sum //last row is directly returned, dont need semicolon
}