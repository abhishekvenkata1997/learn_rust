
pub trait Draw {
    fn draw(&self);
}


pub struct Screen{
    pub components: Vec<Box<dyn Draw>> //dyn keyword dynamic dispatch with trait name and that is held by a smart pointer
}

impl Screen {

    pub fn run(&self) {
        for comp in self.components.iter()
        {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}",self.label);
    }
}

//both are same right, NO => when u use generics we are limited to 1 type, screen components can store either a
//button or slider or input box but it cant store a mixture of them. 

/*pub struct Screen<T: Draw> {
    pub components = Vec<T>
}

impl<T> Screen<T> 
where T: Draw {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}
*/
