fn main() {
    println!("Hello, world!");

    
    {
        let s1 = String::from("hello");
        let s2 = s1 ; 

        println!("{s1} world"); //error , 

        /* but why  ?  
        s1 and s2 point to the same location on the heap memory
        in rust , we dont copy the value of the string in the heap 
        we just create a copy of the pointer , the length , and the capacity 
        because it is more expensive to copy the whole thing  in the heap if the 
        string has a length of 100 for example.
        so , when we reach the curly braces (end of scope) , rust called the function drop 
        that free the memory in the heap , so if we have two pointers , this will 
        be a double free allocation (and this is bad ), because we try to free two pointers
        that point to the same location on the heap , so rust consider s1 invalid 
        after the copy to s2 to avoid double free allocation !*/ 

        /* in integer , this action is valid , because it has a fixed size
        and known so it will be stored in stack directly */

        //owernship use with functions
        let s  = String::from("Take this bro ") ; 
        takes_ownership(s);

        let y : u32 = 1 ; 
        makes_copy(y); 

        println!("{s}");//error cuz it is moved
        println!("{y}");// no error ! vaild cuz of copy trait

        //returning values

        let s1 = gives_ownership();//function moved its content to s1 
        let s2 = String::from("hello"); //s2 in scope
        let s3 = takes_and_gives_back(s2); //s2 moved into the function , the return of the function will be moved to s3 
    } // s1 will be dropped (end of scope) , s2 nothing happend cuz it is moved , s3 will be dropped also (end of scope)



    //reference and borrowing 

    let sb = String::from("hello");
    let len = calculate_length(&sb); //refer to some value without taking an ownership of it 
    println!("the length of {sb} is {len}");

    let mut a = String::from("hello");
    // u can only make one reference to a value when it is muttable
    change(&mut a);

    let mut   b = String::from("ok");

    let a = &mut b ; 
    println!("{a}");
    

    let mut c = String::from("no data race"); 

    {
        let a = &mut c ; 
    }

    let d = &mut c;//we can refer to it because a is dropped


    {
        let mut a = String::from("Hello");
        let b = &a ; 
        let c = &a ; 
        println!("{a},{b},{c}");

        let d = &mut a ; 
        println!("{d}");

        //this is accepted , because b and c are not used after the borrow of the mutable a to d

    }

    //dangling pointers (reference to nothing )

    let ref_to_nothing = dangle();
    println!("{ref_to_nothing}");


 
    //Slice type : sync data like keep updating with the current state of string , example : u have a string , u calculate its length , and after that u clear the string , but length stays , so the length is not updated with the new state of the string , u can apply slices on arrays also
 {
     let mut s = String::from("Hello world");
     let slice0 = &s[..]; // take ref of the whole string
     let slice1 = &s[1..]; // take ref of char1 to the end
     let slice2 = &s[..4]; // take ref of char0 to 4 

     //application 

     let word = do_slices(&s);//immutable borrow
     s.clear();//mutable borrow 
     println!("the first word is:{word}");//use the immutable , error cuz mutable is used and this is immutable borrow call 

     let my_string = String::from("let's go");

     let word = do_slices(&my_string);// &my_string has a type of &String but func accept a &str param , this is a deref u will learn it insh'allah
 }    


}


fn takes_ownership (some_string:String) {
    println!("{some_string} is moved ");
}

fn makes_copy (integer:u32) {
    println!("{integer} still can be used cuz it has the copy trait");
}

fn gives_ownership () -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back (a_string : String ) -> String {
    a_string
}

fn calculate_length (s:&String ) -> usize {
    s.len()
}

fn change (s : &mut String){ //must be muttable to make changes 
s.push_str("world");
}

fn dangle()-> &String {

    let s = String::from("reftonothing");
    &s
}// we return a reference , but here is the end of scope so there is a deallocate , a reference to nothing occur , to prevnet this , we just return String and s without references (so we use ownership)

fn do_slices (s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i,&item) in bytes.iter().enumerate() {
    if item == b' ' {
        return &s[0..i];
    }
  }
  &s[..]
}