use object_traits::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self){
        println!("Drawing a select box: {:?}",self.options);
    }
}

pub struct text_field {
    size: u32,
    value: String
}


fn main() {
    let screen = Screen{
        components : vec![
            Box::new(SelectBox{
                width: 10,
                height: 5,
                options: vec![
                    String::from("man"),
                    String::from("woman"),
                    String::from("none")
                ]
            }),
            Box::new(Button{
                width: 5,
                height: 4,
                label: String::from("Click me")
            }),
            /*Box::new(text_field{
                size: 5,
                value: String::from("name of the user")
            }) draw type is not implemented for this struct hence using that will give you a not
            implementing draw trait error */

        ]
    };

    screen.run();
}


//static vs dynamic dispatch
/*metamorphisation -> use generics and strait bounds (process where compiler will generate non generic implementations of
functions based on concrete types used in place of generic types) -> create function with floating point and integers
compiler will generate function two functions called integer add and float add, 
replace generic implementation and substituting it with a concrete implementation
this is called static dispatch -> Compiler knows what static functions you are calling at compile time

Dynamic dispatch -> Compiler has no idea of concrete methods at compile time
these are figured at run time
When using trait objects we need dynamic dispatch
Because we dont know all the concrete objects at compile time
compiler will add code that will be needed to use all variables we could use at run time -> doesnt know all concrete objects needed at compile time
add all to figure out what function to run at run-time
upside is we can write flexible code that can implement with any object that implements a certain trait
object safety -> Only make object safe traits into trait bounds
Two rules
1. Trait is object safe when all the methods implemented on the trait have these two properties
 -> return type is not self and there are no generic parameters in the return value
  -> if trait does not have these two properties then rust cant figure out the type and so cant call the right method  /