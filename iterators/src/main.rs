/*pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    //methofs with default implementations eluded
}*/

//creating my own iterator
struct Counter{
    count: u32
}   

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    } //everytime new counter is created its count is set to zero
}

//implementing a trait for counter
impl Iterator for Counter {
    type Item = u32; //associated type 

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }

}

#[test]
fn calling_next() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn other_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1)) //: impl Iterator<Item = (u32...)>
                    .map(|(a,b)| a*b)// : impl Iterator<Item = u32>
                    .filter(|x| x%3 == 0)//: impl Iterator<Item = u32>
                    .sum();
    assert_eq!(sum,18);


}

#[test]
fn iterator_dem() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.into_iter(); //mutable because next needs a mutable reference to the iterator
    //v1.iter_mut() for mutable reference
    //v1.into_iter() for specific type
    assert_eq!(v1_iter.next(),Some(1));
    assert_eq!(v1_iter.next(),Some(2));
    assert_eq!(v1_iter.next(),Some(3));
    assert_eq!(v1_iter.next(),None);
}




#[derive(Debug,PartialEq)]
struct Shoe{
    size: u32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests{
    use super::*;

#[test]
fn filters_by_size() {
    let shoes: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker")
        },
        Shoe {
            size: 12,
            style: String::from("sandals")
        },
        Shoe {
            size: 11,
            style: String::from("boots")
        }
    ];

    let v3: Vec<Shoe> = shoes_in_my_size(shoes,11);

    for shoe in &v3 {
        println!("Shoe is: {:#?}",shoe);
    }
    assert_eq!(v3,vec![Shoe{
        size: 11,
        style: String::from("boots"),
    }])
}

#[test]
fn iterator_sum() {

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); //methods that consume the iterator(add all nexts and return the sum)
    assert_eq!(total,6);
}

#[test]
fn iterator_map() {
    let v1: Vec<i32> = vec![1,2,3];
    let v1_iter = v1.iter();
    //let v2: Vec<_> = v1.iter().map(|x| x+1); //produce other iterators wont work because we are not doing anything to the map
    let v3: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v3, vec![2,3,4]);
}   
}   

fn main() {
    println!("Hello, world!");

    let v1 = vec![1,2,3];

    let v1_iter = v1.iter(); //lazy, waits for implementation

    for value in v1_iter{
        println!("{}",value);
    } //no specification on how to iterate, logics are encapsulted, they all follow the iterator trait
}

//allow to iterate over elements regardless of how they are stored