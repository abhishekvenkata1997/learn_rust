use hello_macro::HelloMacro; //macro called hello macro implements a trait called hello macro, 
//which will have associated function default implementation that prints out hello macro
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)] //implement hello macro trait for the panccake struct

struct Pancakes;


fn main() {
    Pancakes::hello_macro();
}

//custom derived macros only work on structs and enums
// Attribute like macros

#[route(GET,"/")]
fn index() {
    //...
    //map a HTTP request to a given function
}

#[proc_macro_attributes]
pub fn route(
    attr: TokenStream, //GET, "/"
    item: TokenStream //fn index(){}
) -> TokenStream {
    //...
}

//function like macros
let sql = sql!(SELECT * FROM posts WHERE id = 1)

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    //...
    //look like functions but work on code
}