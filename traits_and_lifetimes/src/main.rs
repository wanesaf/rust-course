use std::{fmt::Display, result};

pub trait Summary {
    fn summarize_author(&self) -> String {
        format!("{}", self.summarize_content())
    } //when u want to implement like two functions , the functions non implemented must call one function from the others functions
    fn summarize(&self) -> String {
        format!("{}", self.summarize_content())
    }
    fn summarize_content(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_content(&self) -> String {
        format!("{}", self.content) // override the default behavior
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({}) ", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_content(&self) -> String {
        // no need to implement all the other functions , becuase summarize called the the function summarize_author
        format!("{}", self.content)
    }
}

pub struct  Pair <T> {
    x : T , 
    y : T , 
}

impl <T> Pair <T> {
    fn new (x : T , y : T) -> Self {
        Self { x , y }
    }
}

impl <T:Display + PartialOrd> Pair <T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
pub trait  noMatterWhat  {
    fn noo () {}
}
pub struct noMatter<T> {
    x  : T , 
    y : T  
}

impl <T : Display> noMatterWhat for T {} // this means where are implement noMatterWhat for every T type that implements the Display trait

#[derive(Debug)]
struct ImportantExcerpt <'a> {
    part : &'a str,
}

impl <'a>ImportantExcerpt<'a> {
    fn announce_and_return_part(&self,announcement:&str)->&str {
        println!("Attention please:{announcement}");
        self.part
    }
}// first , first rule applied (lifetimes elision) => &'a self, &'b str , then we have inputs that contain &self => third rule (give the same &self lifetime to the output )  

fn main() {
    let tweet = Tweet {
        username: String::from("sx_78"),
        content: String::from("Hello World"),
        reply: false,
        retweet: false,
    };
    let summary = tweet.summarize(); // even summarize not implemented , but summarize call the function summarize_author that is implemented
    println!("{summary}");

    // using functions they have traits as params 
     println!("{}",notify(&tweet));

     let pair = Pair::new(1, 2); 
     pair.cmp_display();

     {
        let x = 5 ; 
        let r = &x ; 
        println!("{r}");
     }

     {
        let string_1 = String::from("Hello"); 
        let string_2 = "World";
        let string_3 = longest(string_1.as_str(),string_2); 
        println!("{}",string_3);
     }

     {
        let string1 = String::from("long string is long");
        let result;
        let string2 = String::from("xyz");
        {
            result = longest(string1.as_str(), string2.as_str());
        }// result isnt dropped ? is this because the lifetime of result = end of outer scope min(lifetime(string1),lifetime(string2))
        println!("The longest string is {result}");
    }

    {
        let novel = String::from("Haha. U the goat"); 
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part : first_sentence,
        };
        print!("{:#?}",i)
    }

     
}

//traits  are similairs to interfaces in java

pub fn notify(item: &impl Summary) -> String {
    format!("{}", item.summarize())
}

pub fn notfiy_using_generics <T : Summary > (item : &T) -> String {
format!("{}",item.summarize())
}//same as the notfiy function but more verbose

pub fn notfiy_more <T:Summary> (item1 : &T , item2 : &T){}

pub fn notify_using_display (item : &(impl Summary+Display)) {}

pub fn notfiy_using_generics_and_display <T:Summary+Display> (item1 : &T , item2 : &T){}

pub fn notify_usig_more_than_one_generic < T , U > (item : &T, item2 : &U) 
    where 
        T : Display + Clone  ,
        U : Display 
        {} // using this is more organize when u use more than one generic

pub fn returning_summrizable (item : i32) -> impl Summary {
    Tweet {
        username : String::from("abcdef"), 
        content : String::from("Hello World"),
        reply : false, 
        retweet : true 
    }
}//returning an object that impl a summary trait without specifying the type of the return 

//lifetimes
pub fn longest <'a> (x : &'a str , y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

fn longest_2 <'a>  (x : &str , y : &str) -> String {
    let resulta = String::from("Hello World");
    resulta
}



fn max <'a> (x : &'a i32 , y : &i32) -> &'a i32 {
    x
}

fn first_word  ( s  : &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        } 
    }
    &s[..]
} // no need to specify lifetimes in this case , (lifetime elision) because we have only one input so the compiler
// will assign the same lifetime 'a to input (first rule) and output (2nd rule in lifetime elision)


fn lognest_with_an_announcement<'a,T>(
    x: &'a str, 
    y: &'a str,
    ann: T,
) -> &'a str 
where 
  T : Display, 
  {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
  } // implementing traits that using generics and also definig lifetimes