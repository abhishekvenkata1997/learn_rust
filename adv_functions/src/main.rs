fn add_one(x: i32) -> i32 {
    x+1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32{
    f(arg) + f(arg)
    //unlike closures fn is a type and not a trait
}
//a value f that is a function that takes an integer and returns an integer
//calling function as a parameter inside another function
//already defined function need to be called several times or similar use cases

//better to write do_twice to also take a closure as parameter if necessary
fn do_twice_wclosures<T>(f: T,arg: i32) -> i32
where T : Fn(i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);

    let num_list = vec![1,2,3,4,5];
    let string_list: Vec<String> = 
        num_list.iter().map(|i| i.to_string())
        .collect();

    println!("{:?}",string_list);
}

//return closures from functions
fn return_closure() -> impl Fn(i32)->i32 { //return a function that takes i32 as input and returns i32 as output
    |x| x+1
}

//if we plan on returning two different closures we cant have return type as above
//we need a box with a dyn that is a trait object as return type
fn return_closure2(a: i32) -> Box<dyn Fn(i32)->i32> {
    if a>0 {
        Box::new(move |b| a+b)
    } else {
        Box::new(move |b| a-b)
    }
    //each closure is unique cant have single return type and return two diffeerent closures
}

//Three closure traits -> Fn, FnOne and FnMut
//Fn -> capture variables immutably
//FnOnce -> Takes ownership and consumes variable