struct  User {
    active : bool , 
    username : String ,  
    email : String ,   
    sign_in_count : u64, 
}

#[derive (Debug)]
struct Rectangle {
    width : u32 , 
    height : u32 , 
}

impl Rectangle { 
    fn area (self:&Self) -> u32  { // self is the instance of the struct the method is being called on 
        self.width * self.height
    }
    fn width (self:&Self) -> bool {
        self.width > 0 
    }
    fn can_hold (self:&Self , rec : &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height 
    }
    fn square (width : u32 , height : u32 ) -> Self { // associated function but not a method cuz it doesnt need self as param , tekder tkol constructeur pisk new isnt built in rust
        Self {
            width  , 
            height ,  
        }
    }
}

struct Color (i32,i32,i32); 
struct Point  (i32,i32,i32);

fn main() {
    let mut user1 = User {
        active : true , 
        username : String::from("someusername"),
        email : String::from("someone@example.com"),
        sign_in_count : 1,
    };

    user1.email  = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("haha@example.com"), String::from("souhil")); 

    let user3 = User {
        email : String::from("user3@example.com"),
        ..user1 // other field as the same as the user1 (must come last)
        // u can no longer access to user1 field cuz they are moved (the new owner of the user1's fields is user3)
    };
     
     // dont forget that the other types like int wont moved cuz they had the copy trait so we still can access to these fields if we dont move the string fields (they will be moved)
 
    let user4 = User {
           username : user3.username  , // new owner of user3.username , so u can no longer access to user3  
           active  : user1.active , 
           email: String::from("another@example.com"),
           sign_in_count : user1.sign_in_count       
    };

    //tuple structs 
    let black = Color (0,0,0);
    let origin = Point (0,0,0); 


    // useful when u want to give a name to ur tuples like differ them from other tuples 
    {
       let rec1 = Rectangle {
        width : 40 , 
        height : 50 
       }; 
       let rec2 = Rectangle {
        width : 20 , 
        height : 30 
       }; 
       let rec3 = Rectangle {
        width : 50 , 
        height : 20 
       }; 

       let area =  rec1.area(); 
       println!("area is {area}"); 
       println!("rec1 is {rec1:#?}");// using the debug trait , u can also use the dbg! macro
       
       if rec1.width() {
        println!("Width is postive and its value is {}", rec1.width);
       }

       println!("can rec1 hold rec2 ? {}", rec1.can_hold(&rec2));
    }

    //methods     

    let rec1 = Rectangle::square(3, 4) ; 
    println!("rec1 has {rec1:#?}");
    
}

 fn build_user (email:String , username:String) -> User {
    User {
        active : true , 
        username,// name of params is the same as the field so use shorthand init  
        email , 
        sign_in_count : 1 , 
    }
 }
