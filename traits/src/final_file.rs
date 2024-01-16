

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {

    fn Summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn Summarize(&self) -> String {
        format!("{}, by {}",self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn Summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    /*fn Summarize(&self) -> String {
        format!("{}: {}",self.username, self.content)   
    }*/
}

//show shared behavior between tweet and news article
pub trait Summary {

    //summarize the author
    fn Summarize_author(&self) -> String;

    // summarize provides default implementation but tweet and news article
    fn Summarize(&self) -> String {
        format!("(Read more from {}...)", self.Summarize_author() )
    }

}

pub fn returns_summerizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know of all, people"),
        reply: false,
        retweet: false
    }
}
