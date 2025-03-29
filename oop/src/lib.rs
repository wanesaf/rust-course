pub struct Post {
    state : Option<Box<dyn State>> , 
    content : String 
}

impl Post {
    pub fn new () -> Post {
        Post {
            state : Some(Box::new(Draft{})),
            content:String::new()
        }
    }
    pub fn add_text (&mut self,content : &str) {
        self.content.push_str(content);
    }
    pub fn request_review (&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()); 
        }

    }
    pub fn approve (&mut self ) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve()); 
        }
    }

    pub fn reject (&mut self ) {
        if let Some (s) = self.state.take() {
            self.state = Some(s.reject()); 
        }
    }
    pub fn content (&self)-> &str {
       self.state.as_ref().unwrap().content(self)
    }
}

trait State  {
    fn request_review(self : Box<Self>) -> Box <dyn State>;
    fn approve (self : Box<Self>) -> Box <dyn State>;
    fn reject (self : Box<Self>) -> Box <dyn State>; 
    fn content <'a> (&self , post :&'a Post ) -> &'a str {
      "" // we do this to not implement these function on draft and pending review
    }
}



struct Draft {} 

impl State for Draft {
    fn request_review(self : Box<Self>) -> Box<dyn State> { // Box here meaning that the function call is correct only when we call the function on a variable that is in a box 
        Box::new(Pendingreview{})
    }
    fn reject (self : Box<Self>) -> Box<dyn State> {
        self 
    }
    fn approve (self : Box<Self>) -> Box<dyn State> {
        self 
    }
}

struct Pendingreview {}

impl State for Pendingreview {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject (self : Box<Self>) -> Box<dyn State> {
        Box::new(Pendingreview{})
    }
    fn approve (self : Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {} 

impl State for Published {
    fn request_review(self : Box<Self>) -> Box<dyn State> {
        self 
    }
    fn approve (self : Box<Self>) -> Box<dyn State> {
        self 
    }
    fn reject (self : Box<Self>) -> Box<dyn State> {
        self
    }
    fn content <'a> (&self , post :&'a Post ) -> &'a str {
        &post.content   
    }
}