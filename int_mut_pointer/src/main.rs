
//Rc<T> allows multiple owners to borrow same data immutably
//Box<T> and RefCell<T> have single ownership
//Box<T> allows immutable or mutable borrow checked at compile timer
//RefCell<T> allows mutable or immutable borrow checked at run time
//Because RefCell<T> allows mutable borrow checked at run time, 
//you can mutate the value inside the RefCell<T> even   when the RefCell<T> is immutable

pub trait Messenger {
    fn send(&self, msg: &str);
}


//T is borrowed value, hence messenger has same lifetime as that of T
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

//adding lifetime to impl because T is borrowed value
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage_of_max: f64 = self.value as f64 / self.max as f64;

        if percentage_of_max >=1.0 {
            self.messenger.send("Error: You have exceeded the limit/ quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent Warning: You've used up 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used upto 75% of yout quota");
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: RefCell::new(vec![]) 
            }
        }
    }

    //we cannot have two immutable reference at the same time checked at run time
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            //let mut two_borrow = self.sent_messages.borrow_mut(); -> cant have another mutable sent_messages inside mockMessenger
            one_borrow.push(String::from(message)); //only reference can be made and data can be pushed using refcell
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_over75_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(),1);
    }
}

fn main() {
    
    let a = 5;
    // let b = &mut a; -> cannot mutably borrow a because a is immutable

    let mut c = 10;
    let d = &c; /*d is taking immutable borrow of a mutable variable*/
    /* *d = 20 -> d is an immutable reference so we cannot change its value */

}
