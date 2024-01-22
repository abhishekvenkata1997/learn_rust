
#[macro_export] //when brought in scope make it available
macro_rules! vector_abhishekvenkata { //macro_rules followed by name of macro
    //vec![1,2,3]
    ( $($x:expr),* ) => { //curlt braces to show body of our macro => match expression with one arm, pattern to match on
        //and a code expression that gives output or return value
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x); //$ before arm or arrow matches and runs that x inside this arrow part
        )*
        temp_vec
    };
}
//($x: expr),* 1. Like regular expressions x can be any rust expression, 
//has comma between each value of x and * meaning it can occur 0 to inf number of times