
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeOrder(i32, i32, i32)
}

//see what takes most space, space of enum is equal to the one that takes max size

enum List{
    Cons(i32, Box<List>), //tuple that holds an integer and a List
    Nil
} //recursive enum
//rust needs to know space taken at compile time
//in theory we could recurse forever
//acts a little bit like linked list

//cant determine size taken by this list since its recursive
//so store a smart pointer, on stack its just a pointer so its fixed size

use List::{Cons, Nil};

fn main() {
   
   let b: Box<i32> = Box::new(5); //5 is what we want to store on the heap -> stack stores pointer to the 5 to point to the value 5 at heap
    println!("b = {}",b);

    //1. Whose exact size cant be known at compile time, but we need its size at a later point in time
    //2. When we want to extract data but not copy or clone it into another variable
    //3. The value must only implement a specific trait but does not need to be of a specific variable type

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    //using a box as a smart pointer instead of a recursive, because pointer takes fixed size

}
