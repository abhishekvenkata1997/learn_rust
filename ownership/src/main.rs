mod main2;
mod ref_main;
use main2::main2;
use main2::main3;
use main2::main4;
use ref_main::ref_main;
fn main() {
    

    fn a(){
        let x = "hello";
        let y = 22;
        b();
    }

    fn b(){
        let x = String::from("world"); 
    }
    main2();
    main3();
    main4();
    ref_main();
}
