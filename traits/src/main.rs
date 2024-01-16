mod final_file;
use final_file::*;
use final_file::returns_summerizable;
use final_file::Summary;
use std::fmt::Display;


//taking a reference to an item that implemenets Summary(Can be tweet or a news article)
pub fn notify(item: &impl Summary){
    println!("Breaking News! {}", item.Summarize());
}

//writing a generic type of type summary, restricting parameter to be of a reference to that type
//longer form for complex cases using trait bounds
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news in Hyderabad! {}", item.Summarize());
}

/* 
pub fn notify3(item1: &(impl Summary+Display), item2: &impl Summary) {
    //..
}


pub fn notify4<T: Summary + Display>(item1: &T, item2: &T) //makes sure both item1 and item2 must be of the same type
{
    //..
}

pub fn some_function<T:Display + Clone, U:Clone + Debug >(t: &T, u: &U) -> i32 {
        //...
    1
}


pub fn some_function2<T,U>(t: &T, u: &U) -> i32 {
    where T: Display + Clone,
          U: Clone + Debug
    1
    //...
}

*/

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
} //all structs have access to these functions


impl<T: Display + PartialOrd> Pair<T> {
    
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", &self.x)
        } else {
            println!("The largest number is y = {}", &self.y)
        }
    }
} //only some structs have access to this function


//blanket implementations -> Implement a trait ona type that implements another trait 
impl<T: Display> ToString for T {
    // --snip--
}
//use ToString trait on all traits that have the display trait in them

fn main() {

    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling.")
    };

    println!("Tweet Summary {}",tweet.Summarize());
    println!("Article Summary {}", article.Summarize());
    notify(&article);
    println!("Hello, world!");

    println!("{}", returns_summerizable().Summarize());
}
