mod final_structs;
use final_structs::struct_main;
mod final_enums;
use final_enums::enum_main;
mod final_structs2;
use final_structs2::struct_main2;
mod final_generic_p;
use final_generic_p::genp_main;
fn main() {
    
    let num_list = vec![34, 60, 28, 18, 54];

    let char_list = vec!['a','b','c','d'];
    let mut largest = get_largest_generic(num_list);

    let mut largest_char = get_largest_generic(char_list);

    println!("Largest number is {}  ",largest);
    println!("Largest character is {}", largest_char);
    struct_main();
    enum_main();
    struct_main2();
    genp_main();
}


//Using generics instead of single type characters
//T(Type) is restricted to a character that has ordering and an ability to copy it elsewhere
fn get_largest_generic<T: PartialOrd + Copy>(num_list: Vec<T>) -> T {

    let mut largest = num_list[0];

    for number in num_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

//need to specify type if you want the function to only take specific type
fn get_largest(num_list: Vec<i32>) -> i32 {
    
    let mut largest = num_list[0];

    for number in num_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

