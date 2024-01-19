
struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    
    fn drop(&mut self) {
        println!("Dropping customerSmartPointer with data `{}`",self.data);
    }
}


fn main() {
    
    let c = CustomSmartPointer{
        data: String::from("my stuff")
    };

    let d = CustomSmartPointer{
        data: String::from("other stuff")
    };
    drop(c); //this can be called whenever we want
    //drop pointers are auto created and called when main function ends
    println!("Custom Smart pointers created!");

}
