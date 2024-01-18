//! # Art
//!
//! A library modeling artistic concepts
 

///Adds one to the number given
/// 
/// #Examples
/// 
/// ```
/// 
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(answer,6);
/// 


pub mod kinds{

    /// The primary color according the RYB color model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    ///Combines two primary colors in equal amount to create 
    /// a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here   
        SecondaryColor::Orange
        //ANCHOR: here
    }
}

pub fn add_one(args: i32) -> i32 {
    args+1
}