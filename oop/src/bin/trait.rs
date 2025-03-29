pub trait draw { 
    fn draw (&self) {}
}

pub struct Screen {
    pub compenents : Vec<Box<dyn draw>> // trait object : any type inside the box that implements the draw trait , allow for multiple types at runtime
}

impl Screen {
    pub fn run (&self) {
        for compenent in self.compenents.iter() {
            compenent.draw(); 
        }
    }
}

pub struct Button { 
    pub width : i32 , 
    pub height : i32 , 
    pub label : String,
}

impl draw for Button {
    fn draw (&self) {}
}

pub struct SelectBox {
    pub width : i32 , 
    pub height : i32 , 
    pub options : Vec<String>
}

impl draw for SelectBox {
    fn draw (&self) {}
}

fn main () { 
   let button = Button {width : 1 , height : 1 , label : String::from("Exit") } ; 
   let select_box = SelectBox {
    width : 1 , 
    height :  1 , 
    options : vec![String::from("Mercedes"),String::from("Audi"),String::from("BMW")] 
   };
   

   let screen = Screen {compenents : vec![Box::new(button),Box::new(select_box)]};
   screen.run();
}