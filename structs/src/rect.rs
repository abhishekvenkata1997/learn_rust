#[derive(Debug)] //trait, provides basic implementation of debug trait
struct Rect {
    height: u32,
    width: u32
}

//you can have multiple implementation blocks for each struct
//other than methods we have something else called associative functions describeds in the next implementation
impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    //function tied to the struct implementation always use self keyword to refer to the existing value that function is using

    fn can_hold(&self, other: &Rect) -> bool {
        if self.width > other.width && self.height > other.height {
            return true
        } else {
            return false
        }
    }
}

impl Rect {
    //no self arguments are passed
    fn square(size: u32) -> Rect {
        Rect{
            height: size,
            width: size
        }
    }
}


pub fn main2() {

    let width = 30;
    let ht = 30;

    let rect1 = Rect{
        height: 100,
        width: 40
    };

    let rect2 = Rect{
        height: 99,
        width: 39
    };
    println!("rectangle is: {:#?}",rect1);
    println!(" Area via impl method {}",rect1.area());

    if rect1.can_hold(&rect2) {
        println!("Yes it can hold");
    } else {
        println!("No other rectangle is bigger");
    }

    let Assoc_rect = Rect::square(24);
    println!("Assoc rect is {:#?}",Assoc_rect);
    println!("Area of the rect is {}", area1(&rect1));
}

// methods are similar to functions but they are tied to the definition of a stuct -> this is a function
pub fn area1(rect1: &Rect) -> u32 {
    rect1.height * rect1.width
}

//normally
pub fn area2(width: u32, height: u32) -> u32 {
    width * height
}