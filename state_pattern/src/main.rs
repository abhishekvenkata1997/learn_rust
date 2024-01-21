mod lib1;
mod lib2;

use lib2::Post as Lib2Post;
use lib1::Post;

fn main() {

    let mut post = Post::new();
    let mut post2 = Lib2Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("",post.content());
    post2.add_text("I ate a salad for lunch today");

    post.request_review();
    assert_eq!("",post.content());

    let post2 = post2.request_review(); 

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let post2 = post2.approve();
    assert_eq!("I ate a salad for lunch today", post2.content());
}

