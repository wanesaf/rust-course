use std::{thread, time::Duration};



#[derive(Debug, PartialEq , Copy , Clone)]
enum ShirtColor {
    Red, 
    Blue,
}

struct Inventory {
    shirts : Vec<ShirtColor>
}

impl Inventory   {
    fn giveway (&self , user_preference : Option<ShirtColor>)->ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked (&self) -> ShirtColor {
        let mut num_red = 0 ; 
        let mut num_blue = 0; 
    
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red+=1,
                ShirtColor::Blue => num_blue+=1,
            }
        }
            if num_red > num_blue {
                ShirtColor::Red
            }else {
                ShirtColor::Blue
            }
    
        }
    }
#[derive(Debug)]
struct Rectangle {
    width : u32 ,
    height : u32 ,
}
fn main () {/* 
 let store = Inventory {
    shirts : vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
 };
 let user_pref1 = Some(ShirtColor::Red);
 let giveaway1 = store.giveway(user_pref1);

 println!("{:#?}",giveaway1);

 let user_pref2 = None ;

 let giveaway2 = store.giveway(user_pref2); 

 println!("{:#?}",giveaway2);

*/

 
 let expensive_closure = |num: u32| -> u32 {
    println!("calcul");
    thread::sleep(Duration::from_secs(2));
    num
 };

 let add_one = |x| x+1; // return x+1
 
 let ex = add_one(3) ; 
 //let ex2_dont_compile = add_one(4.5); // dont compile , because the closure type is i32 because of the first variable used the closure

 let list = vec![1,2,3] ; 

 let only_borrow = || println!("Hello , there is the list {:#?}",list);

 only_borrow();

 println!("after the closure calling , the list still exist ! ,{:#?} ", list); 

 {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");
 }


 {
    let list = vec![1,2,3]; 
    thread::spawn(move||println!("the list is : {:#?} ,",list))
                 .join()
                 .unwrap();
 } // we use move to force the closure to take ownership of the list , ki tl7a9 threads tfham khri 
  let mut list = [
    Rectangle {width : 3 , height : 7 }, 
    Rectangle {width : 1 , height : 5}
  ];

  list.sort_by_key(|r| r.width);
  println!("{:#?}",list);

  
}