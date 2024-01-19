use std::rc::Rc;

//when mutliple variables refer to the same pointer like different lines of a graph pointing to the same node
// we need to wait until the node is dereferenced by all lines that have it
//then we drop that node

enum List {
    Cons(i32,Rc<List>),
    Nil
}

use crate::List::{Cons,Nil};


fn main() {
    //let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //let b = Cons(3, Rc::clone(&a)); //what this does is increase reference count to that specific value
    //let c = Cons(4, Rc::clone(&a));

    //Creating the same we did above but with comments to check reference counts
    let a = Rc::new(Cons(5, Rc::new(Cons(10,Rc::new(Nil)))));
    println!("Count after creating a = {}",Rc::strong_count(&a));

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope  = {}",Rc::strong_count(&a));
    //multiple pointers can only view data, but they cant mutate it, they mutabilityis dicussed in the next pointer chapter
}