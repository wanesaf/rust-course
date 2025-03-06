use std::io::{self};
fn main() {
    let mut a = 0 ; 
    let mut b = 1 ;


    let  input : u32 = loop {

    println!("Enter the number ");
    let mut input = String::new();
    
    io::stdin()
    .read_line(&mut input)
    .expect("Error ");

        match input.trim().parse() {
        Ok(num) =>   break num,
        Err(_) => {
          println!("This is not a  number!");
          continue ; 
        } 
    };
};
println!("The number is {input}");

let mut i = 0 ; 
let mut c = 0 ; 
while  i<input {
    c = a + b ;
    b = a ; 
    a = c ;
    i+=1;
}

println!("The {input}nth fibanocci is {c}");

}
