use std::thread;
use std::time::Duration;

fn simulated_calc(intensity: u32) -> u32 {
    println!(" Calc slolwly.. ");
    thread::sleep(Duration::from_secs(2));
    intensity
}



fn main() {

    let sim_intensity = 10;
    let sim_random_num = 7;
    generate_workout(sim_intensity, sim_random_num);

   let x = 4;
   let eq_to_x = |z| z==x; //closure has an understanding on environment, i.e variables that are in the same scope
  /*  fn equal_to_x(z:i32) -> bool {
    z==x
   }    */ //value of x is not understood in the function but closure understands it.
   let y=4;
   assert!(eq_to_x(y));

   let x = vec![1,2,3];
   //let eq_to_x_vec = move |z| z == x;
   //when we write move along with closures ownershipo of variable goes to closure cant use it elsewhere
   //like in the print statement that follows it
   let eq_to_x_vec = |z| z == x;
   println!("Cant use x here: {:?}",x); 

   let y = vec![1,2,3];
   assert!(eq_to_x_vec(y));

   //FnOnce -> takes ownership of variables inside closures environment, cant take ownership more than once
   //FnMut -> Borrows values mutably
   //Fn -> Borrows value immutably
}

struct Cacher<T> 
where 
    T: Fn(u32) -> u32 //need to use generic and trait bound other bounds are Fn FnMut and FnOnce
{
    calculation: T, //can store a regular function as well in the struct
    value: Option<u32>
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn generate_workout(intensity: u32, random_number: i32) {

    //anonynmous fnuctions instead of input parameters in paranthesis they are in pipes
    //restricting datatypes not necessary
    //whatever type it sees in the first function it accepts it
    //cannot accept multiple types as input unless a generic type like T is specified
    let expensive_closure = |num: u32| -> u32 {
        println!("Calc slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cached_result = Cacher::new( |num: u32| -> u32  {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });


    let example_closure = |x: u32| x; //exmaple of writing closure in one line

    //caching expensive operation and returning new value as saved
    if intensity < 25 {
        println!(" Today do {} pushups, ",
        cached_result.value(intensity));

        println!("Next do {} situps, ",
        cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take rest today");
        } else {
            println!("Today run for {} minutes",
            cached_result.value(intensity));
        }
    }
}
