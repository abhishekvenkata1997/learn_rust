
struct Point<T,U> {
    x: T,
    y: U
}

impl<T,U> Point<T,U> {
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y
        } //this is returned as output
    }
}


pub fn struct_main2() {
    let p1 = Point{x:5, y: 10.4};
    let p2 = Point{x:"Hello",y:'c'};
    let p3 = p1.mixup(p2); //passing two points getting a mix of both points

    println!("p3.x = {}, p3.y={}",p3.x,p3.y);
}