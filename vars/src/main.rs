
fn main () {
    //shadowing 
    /* 
    let x = 5 ; 

    let x = x + 5 ; // x = 10

    {
        let x = x * 2 ; // x = 20 
        println!("{x}");
    }

    println!("{x}"); // x = 10 
    */

    //compouned types 
    
    //tuples
    /* 
    let tup : (u32,f32,u8) = (500,6.4,3);

    let (x,y,z) = tup ; //destruction 

    println!("The value of y is {y} "); 

    let six_four = tup.0 ; 
    println!("The value of x is {six_four}");

    */

    //arrays
    /* 
    let a = [1,2,3,4,5] ; //same type for each element 
    
    let b = [3;5] ; // [3,3,3,3,3]

    let c = a[0] ; 

    println!("{c}");
        


    let array = [5,6,7,8] ; 

    println!("Please type in the index");

    let mut index = String::new(); 

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read the line "); 

    let index : usize  =  index
        .trim()
        .parse()
        .expect("The index entered was not a number "); 

    let element = array[index];

    println!("{element}");

*/


    //functions

    /* 
    let  x = 1;  
    print_func(x);

    let y = {
        let x = 4 ; 
        x + 1 // if u add a semi colon it turn it into a statement and it will not be an expression so y wont be equal to x+1 (rust is insane)
    };

    println!("The value of y is {y}");
    let x = five(); 
    println!("The value of x is {x}");
    
    let  x = add_one(x);
    println!("The new value of x is {x}"); //sra shadowing hna 
    */

    //If_else 

    /* 
    let x = 3 ; 

    if  x > 0 {
        println!("Number sup 0 ");
    }else {
        println!("Number inf 0");
    }

    /*if x {
        println!("number was three") // error , number is not boolean
    }*/

    //using in let statement

    let cond = true ; 
    let x = if cond  {5} else {3} ; 
    println!("value of x is {x}");
*/

    //repeating block of codes using loops

    //dont use this lol 
    /*loop {
        println!("Again ! ");
    }*/



    //returning values from loops 
    let mut counter = 0 ; 

    let result = loop {
        counter += 1 ; 

        if counter == 10 {
            break counter *2;
        }
    };

    println!("Result returning is {result}");
    
    //loops label 'zaeima hadi '
    let mut count = 0 ; 

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10 ;

        loop {
            println!("remaining is {remaining}");
            if remaining == 9 {
                break ; 
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1 ; 
    }

    println!("End count = {count}");    

    //while loops 

    /* 
    let mut x = 0 ;

    while x < 5 {
        println!("{x}");
        x+=1;
    }

    */

    //looping through a collection using for 

    //let a = [1,2,3,4] ; 

    //commun use is for because while can cause panic when index like index = 4 and the start loops , also it is slow because it added verification conditions 

    /*
    for element in a {
        println!("{element}");
    }
    */

    //bonus 
    /* 
    for number in (1..4).rev() {
        println!("{number}");//print m 3 7ta 1 , 4 not included
    }
    */

}

//functions
/* 
fn print_func (x:i32) /* type of variables must be declared */ {
     println!("the value of x is {x}")
}

fn five () -> i32 {
      5 //the return value (the last expression in the function)
}

fn add_one (x:i32) -> i32 {
    x+1
}*/