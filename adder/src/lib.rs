#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

pub struct Guess {
    value: i32
}

impl Guess {
    
    pub fn new(value: i32) -> Guess {
        if value < 1  {
            panic!("value is out of bounds, it must be between 1 to 100 {}",
                    value);
        } else if value > 100 {
            panic!(" Guess value must be less than or equal to 100 {}",
                    value);
        }
        Guess {value}
    }
}
impl Rectangle {
    
    fn can_hold(&self, rect2: &Rectangle) -> bool {

        if self.width > rect2.width && self.width > rect2.width {
            true
        } else {
            false
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a+2
}

pub fn greeting(name: &str) -> String {
    return format!("Hello {}!",name);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 4
        };  

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {

        let smaller = Rectangle {
            width: 5,
            height: 6
        };
        let larger = Rectangle {
            width: 7,
            height: 8
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_wrong() {
        assert_ne!(5,add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greetings did not contain name, value was `{}`",
            result
        );
    } 

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }  

    #[test]
    fn it_works() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            return Err(String::from("two plus two does not equal 4"));
        }
    }

}
