

struct Point {
    x: i32,
    y: i32
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Messageup {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color)  
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}


fn main() {
   
   let msg = Message::ChangeColor(0, 160, 25);

   let msg_up = Messageup::ChangeColor(Color::Hsv(0,160,254));

   let ((feet, inches), Point{x,y}) = ( (3,-10), Point{x: 3, y: -10});

   match msg_up {
    Messageup::Quit => {
        println!("Quit");
    },

    Messageup::Move{x, y} => {
        println!(" Move to x {} and y {}",x,y);
    },

    Messageup::Write(text) => {
        println!("Text message {}",text);
    },

    Messageup::ChangeColor(Color::Rgb(r,g,b)) => {
        println!("Change Color with color enum r: {}, g: {}, b: {} ",r,g,b);
    },

    Messageup::ChangeColor(Color::Hsv(h,s,v)) => {
        println!("Change Color with color enum h: {}, s: {}, v: {}",h,s,v);
    }
   }
   match msg {
    Message::Quit => {
        println!("Quit");
    }
    Message::Move{x, y} => {
        println!(" Move to x {} and y {}",x,y);
    }

    Message::Write(text) => {
        println!("Text message {}",text);
    }

    Message::ChangeColor(r,g,b) => {
        println!("Change Color: red {}, green {}, blue {}", r,g,b);
    }
   }
   let x = 1;

   match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything")
   }

   let x = Some(5);
   let y = 10;

   match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched y  = {:?}",y),
    _ => println!("Default case, x = {:?}",x)
   }
   //match block creates new scope -> items inside match scope shadow the variables outside it  

   let m = 3;

   match m {
    1..=5 => println!("values 1 through 5"), //check for an inclusive range
    _ => println!("Check something else")
   }

   let y = 'c';

   match y {
    'a'..='j' => println!("eatly ASCII Letters"),
    'k'..='z' => println!("Later level ASCII letters"),
     _ => println!("something else")
   }

   let p: Point = Point{x: 0, y:7};

   let Point {x: a, y: b} = p;
   assert_eq!(0,a);
   assert_eq!(7,b);

   match p {
    Point {x,y: 0} => println!("On the x axis"),
    Point {x:0,y} => println!("On the y axis"),
    Point {x,y} => println!("On neither axis: {} {}",x,y)
   }

   foo(3,4); //Ignoring 3 in the function

   let mut setting_value: Option<i32> = None;
   let mut new_setting_value: Option<i32> = Some(10);

   match (setting_value, new_setting_value) {

    (Some(_), Some(_)) => {
        println!("Cannot overwrite an existing customized value");
    }

    _ => { setting_value = new_setting_value; 
            println!("Setting_value is set");}
   }
   println!("Setting is {:?}",setting_value);

   let numbers = (2,4,8,16,32);

   match numbers {
    (first,_,third,_,fifth) => {
        println!("Some numbers: {}, {}, {}",first, third, fifth);
    }
   }

   let _x = 5; //prefixing variable with an underscore
   let y = 10;

   let s = Some(String::from("Hello"));

   /*if let Some(_s) = s{
    println!("Found a string");
   }*/
   if let Some(_) = s {
    println!("found a String");
   }

   println!("{:?}",s);


   struct Point3D {
    x: i32,
    y: i32,
    z: i32
   }

   let origin = Point3D {x: 0, y: 0, z:0};

   match origin {
    Point3D {x, ..} => println!("x is {}",x)
   }

   let x = Some(5);
   let y = 10;

   match x{
    Some(n) if n==y => println!("Matched n= {}",n),
    _ => println!("Default  case, x = {:?}",x)
   }
}

// using _ as first function ignores the first parameter
fn foo(_: i32,y: i32) {
    println!("This code only uses the y parameter {}",y);
}
