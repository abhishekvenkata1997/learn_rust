use std::env;
use std::process;
use mini_grep_iterator::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
    
   /*let config = Config::new(&args).unwrap_or_else(|err| 
    {
        eprintln!("Problem parsing arguments!: {}",err);
        process::exit(1);
   });*/

   let config1 = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}",err);
        process::exit(1);
   });

    println!("Searching for {}",config1.query);
    println!("Searching in file: {}", config1.filename);

    if let Err(e) = mini_grep_iterator::run(config1){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}

