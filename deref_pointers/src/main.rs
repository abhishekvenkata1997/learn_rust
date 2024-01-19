
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
//x is not stored on the heap for the above myBox pointer

impl<T> Deref for MyBox<T> {
    type Target = T; //associated types to define generic parameter
    
    fn deref(&self) -> &T {
        &self.0 //references to first item in the tuple struct
    }
    //when the deref operator is used point to item stored in tuple struct
}
fn main() {

    let x = 5;
    let y = &x;

    let z = Box::new(x);
    let z1 = MyBox::new(x);
    assert_eq!(x,5);
    assert_eq!(*y,5);
    // assert_eq!(y,5); wont work
    assert_eq!(*z,5);
    assert_eq!(*z1,5); //get an error myBox cannot be dereferenced because we only have the new function right now

    let m = MyBox::new(String::from("Abhishek"));
    hello(&m); //&m is a reference to a &MyBox<String> but it works even though input expects an &str 
    //&MyBox<String> -on-deref-> &String -on-deref-> &str -> This auto dereferencing happens auto in rust
    //otherwise we would have to write it as hello(*(&m)[..]);
    //similar to deref trait we can use DerefMut will give us reference to mutable reference
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}