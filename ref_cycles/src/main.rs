
use crate::List::{Cons,Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
//weak is a version of rc that holds a non-owning reference to the allocation
//
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), //to be able to swap one list with another list and to refer them in several places 
    Nil
}

//preventing pointer ownership
#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}


impl List {

    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }

}
fn main() {
    
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}",a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); //points to next item as a

    println!("a rc count after b creation  ={}",Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}",b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a is: {}",Rc::strong_count(&a));
    println!("a rc count after changing a is: {}", Rc::strong_count(&b));

    //Uncomment the next line to see if there is a cycle
    //it will overflow the stack
    //println!("a next item = {:?}",a.tail());


    //describing Node functionalities
    let leaf: Rc<Node> = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("Leaf strong = {}, weak = {}", 
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)) ; 
    println!("Leaf parent: {:?}",leaf.parent.borrow().upgrade()); //immutable reference take weak pointer upgrade to rc poointer
    {
        let branch: Rc<Node> = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)])
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("Branch strong = {}, weak = {}", 
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)) ; 

        println!("Leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)) ;
    }
    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)) ;

}
