use std::fmt::Display;


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
 ) -> &'a str //parameters and return value
where T: Display, //condition on input type
{
    println!("Announcement {}",ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    println!("Hello, world!");
}
