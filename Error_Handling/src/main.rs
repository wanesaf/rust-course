use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
fn main() {
    //recoverable errors using Result<K,V>
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("Error when trying to create the file"),
            },
            other_error=> panic!("Error when trying to open the file"),
        }, 
};

//alternative 
//unwrap
let greeting_file_result  = File::open("hello.txt").unwrap();
//expect 
let greeting_file_result  = File::open("hello.txt")
                                .expect("You may inclue the hello.txt file");
                               
}

fn read_username_from_file () -> Result<String,io::Error> {
    let username_file = File::open("hello.txt"); 
    let mut  username_file = match username_file {
        Ok(file) => file, 
        Err(e) => return Err(e), 
    }; 

    let mut username = String::new(); 
    match username_file.read_to_string(&mut username){
        Ok(_) =>  Ok(username),
        Err(e)=>Err(e),//last expression in function no need to write return 
    }
}

//using the ? operator 

fn read_username_from_file_2 () -> Result<String,io::Error> {
    let mut username_file  = File::open("hello.txt")?;//if Ok then the value inside the Ok will be returned from this expression and the program continue, otherwise we return the Err totally (the function stops and return the err)
    let mut username = String::new(); 
    username_file.read_to_string(&mut username)?;
    Ok(username)

    //the ? operator can be used only when the return type of the function is compatible with the value the ? is used on .
}

fn last_char_of_first_line (text:&str) -> Option<char> {
    text.lines().next()?.chars().last() 
    //using ? to return option 
}
