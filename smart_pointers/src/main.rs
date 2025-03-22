use crate::List::{Cons, Nil};
use crate::List_using_Rc::{Cons as OtherCons, Nil as OtherNil};
use std::{ops::Deref, rc::Rc};
enum List {
    Cons(i32, Box<List>),
    Nil,
} // In the case of Cons(i32,List)
  //how the compiler allocate the size for the variant of the enum
  //the compiler start at choosing the allocSize of the Cons variant , it will be the sizeof(i32)+List , but we dont have the ListSize ,
  //so it will choose the allocateSize another one for the ConsVariant .... -> infinity

#[derive(Debug)]
enum List_using_Rc {
    Cons(i32, Rc<List_using_Rc>),
    Nil,
}
struct myBox<T>(T);

impl<T> myBox<T> {
    fn new(x: T) -> myBox<T> {
        myBox(x)
    }
}

impl<T> Deref for myBox<T> {
    // to implement deref u declare the target and the deref function
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // bach yemchi deref operator lazem t implement deref method bach tredlek ref
        &self.0
    }
}
#[derive(Debug)]
struct CustomPointer {
    data: String,
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping custom pointer with data : {}", self.data);
    }
}
fn main() {
    let b = Box::new(5); // data (5) is stored on the heap , the box (pointer) is stored in the stack
    println!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); //(1,2,3)

    //deref trait application
    {
        let x = 5;
        let y = &x;
        assert_eq!(*y, 5);
        assert_eq!(*b, 5);
    }

    //impl myBox to learn how deref work

    {
        let boxing = myBox::new(5);
        assert_eq!(*boxing, 5);
    }

    //deref coesion => converts a type that implements the deref trait to another type
    //exemple : &String => &str
    //tsra ki tpassi arguememnt lfunction li type taeo machi kima li fi signature tae methode (lazem ykon yimplementi deref trait)
    {
        let x = myBox::new(String::from("world"));
        hello_world(&x); //deref from &myBox<String> -> &String -> &str automatically with rust
                         //this equivalent to hello_word(&(*x)[..])
    }

    {
        let x = Box::new(String::from("Hello World"));
        assert_eq!("Hello World", *x)
    }

    //this in the case of mutability
    //From &T to &U when T: Deref<Target=U>
    //From &mut T to &mut U when T: DerefMut<Target=U>
    //From &mut T to &U when T: Deref<Target=U> the reverse is impossible because of the borrowing rules
    //borrow checker f third case mara7ch yedmen bli ki y7awl & -> &mut rah tkon & the only immutable ref to the data tsma mayrisqich y7awlo

    //Drop trait : drop happen when var goes out of the scope
    {
        let c = CustomPointer {
            data: String::from("Hello World ! "),
        };
        println!("{:#?}", c);
    }
    // the drop function of the Drop trait will be called without explicitly call it

    // if u want to drop it early , use std::mem::drop and not the drop function declared in the trait because rust prevent this (drop of std::mem will also called in the end of main => Double free error)
    // why calling drop from std::mem::drop wont cause a double free !
    // because drop here will take ownership of the value , and then when it goes out of scope it will be dropped automatically from rust

    //Reference counted smart pointer
    //its objective is to enable multiple ownership like a value can be owned by various vars
    //Rc<T> keeps track of the number of references to a value to determine whether the value is still in use , if refs=0 => the value can be cleaned up
    {
        let a: Rc<List_using_Rc> = Rc::new(OtherCons(5,Rc::new(OtherCons(10, Rc::new(OtherNil)))));
         println!("number of refs to a is {}", Rc::strong_count(&a));
        let b = Rc::new(Rc::clone(&a));
         println!("number of refs to a is {}", Rc::strong_count(&a));
        // b and a will share the ownership of the data , number of refs to data = 2
        {
          let c = Rc::new(Rc::clone(&a)); // c,b,a share the ownership , number of refs = 3
          println!("number of refs to a is {}",Rc::strong_count(&a));//number of refs = 3 
        }

        //number of refs = 2 (c is dropped)
          println!("number of refs to a is {}",Rc::strong_count(&a));
       //the data wont be cleaned unless refs = 0
    }
}

fn hello_world(x: &str) {
    println!("hello {}", x);
}
