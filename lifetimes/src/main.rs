#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}//if you are using references as our datatypes

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention Please! {}",announcement);
        self.part
    }
}
//no need to give 'a or lifetimes because &self is there which passes its lifetimes to the output
/*impl<'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention Please! {}",announcement);
        self.part
    }
}*/ //Can also declare like this

fn main() {
   
   let s: &'static str = "I have a static lifetime."; //all string literals have a static lifetime because they are stored in programs binary
   let r;
   {
        let x = 5;
        r = &x;
   } // x is invalidated at this point, hence r is pointing to nothing now
   //at this point r is a dangling reference
   // this happens using a borrow checker
   //LIFETIME of r x ends at } this point
   //println!("r: {}",r); -> this wont work

   let str1 = String::from("abcd");
   let str2 = String::from("xyz");

   let result = longest_lifetime(str1.as_str(), str2.as_str());
   println!("Longest string is {}", result);

   let str3 = String::from("abhishek");
   let result2;
   {
    let str4 = String::from("Sreya");
    result2 = longest_lifetime(str3.as_str(), str4.as_str());
    println!("The second time longest string is {}", result2);
   }
   
   //println!("The third time longest string is {}", result2); //-> str4 does not live long enough

   let novel = String::from("Call me Ismal. Some years ago..");
   let first_sentence = novel.split('.').next().expect("Could not find full stop");
   let i = ImportantExcerpt {
    part: first_sentence,
   };
   println!("{:?}",i);

}


//&i32 -> a reference
//&'a i32 -> an immutable reference with an explicit lifetime
//&'a mut i32 -> a mutable reference with an explicit lifetime
/*fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */
//what we are saying is a relationship between x and y and the return type
//whatever has the smallest lifetime that gets propogated to the return value 
//because we gave all of them  the same lifetime value

//adding a generic lifetime notation -> explain relationship between different lifetimes
fn longest_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//Lifetime of result must be linked to either one of the input parameters that we passed


//1. Each parameter that is a reference gets its own parameter
//2. If there is exactly one input life parameter, that life time is
//   assigned to all output life parameters
//3. If there are multiple input lifetime parameters but one of them is
//   &self or &mut self the lifetime of self is assigned to all output life 
//   parameters
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}