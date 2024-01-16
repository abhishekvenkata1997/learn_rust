
use std::fmt;
use std::fmt::Result;
use std::io::Result as IoResult;  //using different names for functions if they have the same name
use rand::Rng; 
use rand::{CryptoRng, ErrorKind:: Transient};
use std::io::{self, Write};
pub mod front_of_house {
    
    //parent modules are oblivious to the tasks of the child
    //these options are private and not visible or accesible by them
    /* HIDES Implementation from parent of those of the child */
    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}

        //child modules can view and play with the functions of the parent modules
    }

    mod serving{
        fn take_order(){

        }
        fn serve_order(){

        } 
        fn take_payment(){

        }
    }
}

fn serve_order(){

}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();   
        super::serve_order();
    }

    fn cook_order() {

    }

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup, 
        Salad
    }


}

pub fn eat_at_restaurant() {
    
    //Absolute Path
    //crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_struct() {
    let mut meal = back_of_house::Breakfast::summer("New Meal");
    println!("{}",meal.toast);
    println!("{}",meal.seasonal_fruit);
}

pub fn eat_at_restaurant_enum() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

pub use self::front_of_house::hosting; //bringing functions parents module into scope
//if weare bringing structs or enums into scope we give the exact path


pub fn eat_at_restaurant_waitlist() {

    let secret_number = rand::thread_rng().gen_range(1,101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


fn function1() -> fmt::Result {
    //snip
    Ok(())
}

fn function2() -> io::Result<()> {
    //snip
    Ok(())
}

fn function3() -> Result {
    Ok(())
}

fn function4() -> IoResult<()> {
    Ok(())
}


