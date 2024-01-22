    fn main() {
        //match expressions

        #[derive(Debug)]
        enum Language {
            English,
            Spanish,
            Russian,
            Japanese
        }

        let language = Language::English;

        match language {
            Language::English => println!("Hello World"),
            Language::Spanish => println!("Hola Spanish Mundo!"),
            Language::Russian => println!("Russiam hola!"),
            lang => println!("Cannot figure out the language {:?}", lang)
        }
        //-------------------------------
        //conditional if let expressions
        //-------------------------------

        let auth_status: Option<&str> = None;
        let is_admin = false;
        let group_id: Result<u8,_> = "24".parse(); //either unsigned 8 bit integeror an error

        if let Some(status) = auth_status { //if auth_status is a Some variant then that value is stored inside status
            println!("Auth status: {}",status);
        } else if is_admin {
            println!("Auth status: admin");
        } else if let Ok(group_id) = group_id { //if group id is an Ok variant
            if group_id > 30 {
                println!("Auth status: privileged");
            } else {
                println!("Auth status : basic");
            }
        } else {
            println!("Auth status: guest");
        }
        //if let is not an exhaustive conditional search -> need not check for every condition may cause missed outputs
        //-----------------------------------
        //       WHILE IN CONDITION Loops
        //-----------------------------------

        let mut stack = Vec::new();

        stack.push(1); stack.push(2); stack.push(3);

        while let Some(top) = stack.pop() { //loop keeps running until we have a some variant returning
            println!("{}",top);
        }

        //---------------------------------
        // FOR LOOPS
        //---------------------------------
        let v = vec!['a','b','c','d'];

        for (index, value) in v.iter().enumerate() { //using pattern to destruct the tuple
            println!("{} is at index {}",value, index);
        }

        //LET statements
        let x = 5;
        let (x,y,z) = (1,2,3); //or
        let (x,y,_) = (4,5,6);
        //let pattern  = expression explained above
        
        
        //----------------------
        //Function parameters
        //----------------------
        let point = (3,5);
        print_coords(&point);

        let x = 5; //irrefutable patterns

        //Refutable
        let x: Option<&str> = None;
        if let Some(x) = x { //will check if x is None or a Some variant (will work because if let accepts a refutable pattern)
            println!("{:?}",x);
        };

        let y: Option<&str> = None;
        let Some(y) = y; // -> because y is a none variant this will never match the some variant
        //for such cases we need an if let condition with an else because y is a refutable pattern
        if let y = 5 {
            println!("{:?}",y);
        };

        //can only accept irrefutable patterns:
            // function parameters, let statements, for loops


    }

    fn print_coords(&(x,y): &(i32,i32)) {
        println!("Current location: ( {} {} )",x,y);
    }


