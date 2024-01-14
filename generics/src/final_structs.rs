struct Point<T> {
    x: T,
    y: T
}

//implementation block that takes a generic type U, uses its own self as a paramter 
//and returns the value of x
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y //return value doesnt need comma or need to say return
    }
}
pub fn struct_main() {
    let p1 = Point{x: 5, y:10};
    println!("{}",p1.x()); //only x function is available for point p1 because its i32
    let p2 = Point{x: 5.0, y: 10.0};
    println!("{}", p2.y()); //both x and y functions are available beause p2 is of type f64(i.e 2nd implementation)
    //let p3 = Point{x: 5, y: 12.0};
    //need to have two types for each x and y if we want an int and floating as our point variables
}   