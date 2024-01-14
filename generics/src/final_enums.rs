
//Using two most commonly used enums
//One is if we get the right answer
//Other is if we want to return some error
pub fn enum_main() {
    
    enum Option<T> {
        Some(T),
        None
    } 

    enum Result<T,E> {
        Ok(T),
        Err(E)
    }
}