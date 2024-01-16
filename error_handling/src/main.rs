use std::fs::{self, File};
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn main() {
    //panic!("Crash and burn");

    /*enum Result<T,E> {
        Ok(T),
        Err(E)
    }*/

    let f = File::open("Hello.txt");

    let f = match f {
        Ok(File) => File,
        Err(error) => panic!("Error cannot open hello.txt: {:?}",error)
    };

    let f2 = File::open("Hello1.txt").expect("Failed to open Hello1.txt");


    /*let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the Hello.txt file {}",e)
            }, 
            other_error => {
                panic!("Problem opening the file: {:?} ",other_error)
            }
        }
    };*/ //better way to do multiple match  open if file exists or create it, if some other error shows up print that error
    //a();


    //error propogation -> Send error back to caller
    fn read_username_from_file() -> Result<String, io::Error> {
        
        let f3 = File::open("Hello2.txt");

        let mut f3 = match f3 {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f3.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e)
        }

    }
    
    fn read_username_from_file_simpler() -> Result<String, io::Error> {

        let mut f3: File = File::open("Hello2.txt")?;

        let mut s = String::new();

        f3.read_to_string(&mut s);
        Ok(s)
    }

    fn read_username_from_file_simplest() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello2.txt")?.read_to_string(&mut s)?;
        Ok(s)

        //can just write one line
        //fs::read_to_string("hello.txt");
    }

    //using ? in main function
    let f = File::open("hello.txt")?;
}

fn a() {
    b();

}

fn b() {
    c(23);
}

fn c(num: i32){
    if num==22 {
        panic!("Dont pass 22");
    }
}


