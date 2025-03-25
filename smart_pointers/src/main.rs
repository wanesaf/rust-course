use crate::List::{Cons, Nil};
use crate::ListUsingRc::{Cons as OtherCons, Nil as OtherNil};
use crate::List2::{Cons as anotherCons , Nil as anotherNil};
use crate::ListUsingRcAndRefCell::{Cons as LastCons , Nil as LastNil}; 
use std::cell::RefCell;
use std::{ops::Deref, rc::Rc};
enum List {
    Cons(i32, Box<List>),
    Nil,
} // In the case of Cons(i32,List)
  //how the compiler allocate the size for the variant of the enum
  //the compiler start at choosing the allocSize of the Cons variant , it will be the sizeof(i32)+List , but we dont have the ListSize ,
  //so it will choose the allocateSize another one for the ConsVariant .... -> infinity

#[derive(Debug)]
enum ListUsingRc {
    Cons(i32, Rc<ListUsingRc>),
    Nil,
}

#[derive(Debug)]
enum ListUsingRcAndRefCell {
    Cons (Rc<RefCell<i32>> , Rc<ListUsingRcAndRefCell>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(i32,RefCell<Rc<List2>>),
    Nil 
}

impl List2 {
    fn tail (&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            anotherCons(_,item)=> Some(item),
            anotherNil => None,
        }
    }
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

pub trait Messenger {
    fn send (&self , msg :&str) {}
}

pub struct LimitTracker<'a, T :Messenger> {
    messenger : &'a T , 
    value :  usize ,
    max : usize ,  
}

impl <'a,T> LimitTracker<'a , T>
where 
    T : Messenger,
    {
        pub fn new (messenger : &'a T , max : usize ) -> LimitTracker<'a,T> {
            LimitTracker { 
                 messenger ,  
                  value: 0 ,  
                   max , 
        }
    }
    pub fn set_value (&mut self , value : usize) {
     self.value = value ; 
     let percentage_of_max = self.value as f64 / self.max as f64; 
        if percentage_of_max >= 1.0 {
            self.messenger.send("You are over your quota !");
        }
        else if percentage_of_max >= 0.9  {
            self.messenger.send("Urgent warning : You have used up over 90% of ur quota");
        }else if percentage_of_max >= 0.75 {
            self.messenger.send("Urgent warning : You have used over 75% of ur quota ");
        }
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
        let a: Rc<ListUsingRc> = Rc::new(OtherCons(5,Rc::new(OtherCons(10, Rc::new(OtherNil)))));
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

    //Note : u can mutate data when u have immutable refs to this data , using  unsafe ; this is normally not accepted by the compiler
    //RefCell<T> : single ownership over the data   
    // it's useful when u sure ur following the borrwoing rules but the compiler reject it because it cant understand it 
    //There are situations when a value must mutate itself in its methods but it appears immutable in other code . RefCell is here to acheive this (interior mutability)
    
    //RefCell<T> combined with Rc<T> to get multiple
    {
        let value = Rc::new(RefCell::new(5)); // the 'node : not really ' that contain 5 will have multiple owners , and they can borrow_mut it

        let a = Rc::new(LastCons(Rc::clone(&value), Rc::new(LastNil)));
    
        let b = LastCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = LastCons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
        *value.borrow_mut() += 10;
        *value.borrow_mut() += 10;
        //It's possible because of Rc (immutable refs) combined with RefCell so tborrowi mutably wekt mat7ab (ostoriya)
        
        
    
        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");
    }

    //memory leaks 
    {
        println!("-----------------------------------------------"); // lahbil wallah 
      let a = Rc::new(anotherCons(5,RefCell::new(Rc::new(anotherNil))));
      println!("a initial rc count = {}",Rc::strong_count(&a)) ; //a->5 , refs_count = 1 
      println!("a next item = {:?}",a.tail()); //RefCell(Rc(Nil))

      let b  = Rc::new(anotherCons(10,RefCell::new(Rc::clone(&a))));// b-> 10 -> 5 -> Nil ; refs_count (a) = 2 

      println!("a rc count after b creation = {}",Rc::strong_count(&a)); //2
      println!("b initial rc count = {}",Rc::strong_count(&b)); //1
      println!("b nex item = {:?}",b.tail()); //5->Nil

      if let Some(link) = a.tail() { //a.tail() = RefCell{Rc(Nil) != None}
        *link.borrow_mut() = Rc::clone(&b); // new list : a -> 5 -> 10 -> 5 -> 10 avec b -> 10
      }
      println!("b rc count after changing a = {}", Rc::strong_count(&b));//2
      println!("a rc count after changing a = {}", Rc::strong_count(&a));//2

      println!("a next item = {:?}", a.tail());//overflow : a->b->a->b.....
    }
}

fn hello_world(x: &str) {
    println!("hello {}", x);
}


#[cfg(test)] 
mod tests {
    use super::*;
    use std::cell::RefCell; 
    struct MockMessenger {
        sent_messages : RefCell<Vec<String>>//create a refcell to strore messages , refcell is immutable , but we can get a mutable refs (borrow mut) after , this is impossible without refcell , because u must declare then Vec<String> as mutable and self also 
    }

    impl MockMessenger {
        fn new () -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send (&self , message : &str ) {
            self.sent_messages.borrow_mut().push(String::from(message));
            {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));

            //there is no compilation error , but there is a runtime error because we get two mutable refs of the same data at the same scope , this is how refcell check errors !
            //there is cost for this benifits however , small penality at runtime for checking borrows 
            }
        }
    } 

    #[test]
    fn it_sends_over_75 () {
        let mock_messenger = MockMessenger::new() ; 
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100); 
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len() , 1) ;
        //borrow => return Ref<T> , borrow_mut => return RefMut<T> , they impl deref trait , so u can treat them like regular refs (using *) 
    } 

}