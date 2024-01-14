
enum Option<T> {
    Some(T),
    None
}


fn genp_main() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);

}   

//at compile time it creates two enums one for i32 and other for f64, this improves performance