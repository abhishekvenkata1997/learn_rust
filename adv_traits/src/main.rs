use std::ops::Add;
use std::fmt;


trait Pilot {
    fn fly(&self);
    fn run();
}

trait Wizard{
    fn fly(&self);
    fn run();
}

trait OutlinePrint: fmt::Display { //writing fmt::Display implies the self that is implementing the outline print trait
    //also needs to have fmt::Display trait implemented for that type
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}","*".repeat(len+4));
    }
}


impl fmt::Display for Point { //implement display trait for outline print to work
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})",self.x,self.y)
    }
}
impl OutlinePrint for Point { //for point to implement OutlinePrint trait it needs implement fmt::Display trait as well
    
}
struct Human;

impl Human {
    fn fly(&self) {
        println!("Waving arms furiously");
    }
    fn run(){
        println!("Running furiously");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
    fn run() {
        println!("This is your captain speaking lets run");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("UP UP AND AWAY");
    }
    fn run() {
        println!("RUN RUN STRAIGHT");
    }
}
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}


struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0*1000))
    }
}
// Default Generic type parameters are used because
//1.  Extend a type without breaking code
//2. Allow customization for specific cases most users wont need
//overloading + operation for a point struct

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


pub trait Iterator{
    type Item;
    //associated types declared in traits
    //can be used inside our functions as a placeholder
    //so only when we implement it we can give a type to it
    fn next(&mut self) -> Option<Self::Item>;
    //generics give ability to have multiple types in our implementation
    //but with associated type when its implemented its necessary to specify a type
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}



struct Counter{

}

impl Iterator2<u32> for Counter{
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl Iterator2<u16> for Counter{
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}
// allows two implementations for Iterator2 because we gave a generic type as the limitation
impl Iterator for Counter {

    type Item = u32; //specifying type
    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

/*impl Iterator for Counter {

    type Item = u16; //specifying type
    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
    //the second one is a conflicting implementation which cannot be used
}*/

fn main() {
    println!("Hello, world!");

    let p1 = Point{x:1, y: 2};
    let p2 = Point{x:2, y: 3};

    let p3 = Point{x:1,y:2} + Point{x:2, y:3};
    assert_eq!(p3,p1+p2);

    let person = Human;
    person.fly();
    Pilot::fly(&person); //running pilot implementation
    Wizard::fly(&person); //running wizard implementation
    Human::run(); //this is an associated function not connected to any item of the struct
    <Human as Wizard>::run();
    <Human as Pilot>::run();
}
