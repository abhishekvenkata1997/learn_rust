use std::fmt;

struct Wrapper(Vec<String>);

//to implement a trait on a type we need the trait or the type to be in our local crate
//since both vector and fmt::Display come from outside 
//we need a wrapper around vector to write an implementation for it

struct Age(u32);
struct ID(u32);
//we want to make sure these values are passed correctly to the functions -> this makes sure we 
//are not passing u32 as the datatypes but structs age and ID as parameters to our functions

impl fmt::Display for Wrapper {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
        write!(f, "[{}]",self.0.join(","))
    }
}

/*fn bar() -> ! {
if we are not sure nothing is being returned makes sense for values like Continue 
}*/
fn main() {
    println!("Hello, world!");

    type Kilos = i32; //type alias for i32
    let x: i32 = 5;
    let y: Kilos = 7; //can add i32 to kilos
    println!("{}",x+y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    //can give type aliases instead of rewriting type every time

    //let s1:str = "Hello there!";
    //let s2:str = "Hello I am Abhishek"//dynamically sized types -> size known at run time
    let s3: &str = "Hi I know what to do"; //stores size and location of the dynamic sized string
    //need to put it behind a pointer and must be behind a Box or Rc pointer

}

fn generic<T: ?Sized>(t: &T) {
    //if size of T is unknown at declaration or compile time 
    //we need to add ?Sized and need to use &T or Box<T> or Rc<t> cant refer to it just as T
}
