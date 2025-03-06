use std::fmt::format;
use std::collections::HashMap;
enum SpreadSheet {
    Int(i32), 
    Float(f64),
    Text(String)
}
fn main() {
    let mut v1 : Vec<i32> = Vec::new();
    let mut v2 = vec![1,2,3]; //i32 is the default type

    print!("{v2:#?}");

    v1.push(1);
    v1.push(2);
    v1.push(3);

    let  second = &v2[1];
    println!("the second element is  {second}");
    v2.push(1);// this works because second is dropped because it is not needed (NLL) 

    // second method 
    let second : Option<&i32> = v2.get(1);

    match second {
        Some(second) => {println!("{second}");},
        None => {println!("no second element");}
    }

    //using the iterator
    for i in &v1 {
     println!("{i}");
    }

    // to hold different types
    let mut v : Vec<SpreadSheet> = Vec::new(); 
    v.push(SpreadSheet::Int(1)); 
    v.push(SpreadSheet::Float(1.5)); 
    v.push(SpreadSheet::Text(String::from("Hello World")));


    //String 
    let s1 = String::from("Hello, "); 
    let s2 = String::from("World");

    let s3 = s1 + &s2 ; // concatenation require the left side must be not borrowed , as a result , s1 will be moved and cant be used then 
    //we use format macro  to concatenate various strings becuase using + is unwieldy
    let s3 = format!("{s2}{s3}"); // the beautiful thing is were taking refs and not ownership

    //indexing Strings in rust is not available 
    // because they encoded in utf-8 so rust cant know the type of the return of indexing if it is bytes , letters .. 
    // crates.io provides many functions to handle this


    // Hashmaps 

    let mut h  = HashMap::new();
    h.insert(String::from("Team1"), 10);
    h.insert(String::from("Team2"), 2);

    let team_name = String::from("Team1");
    let score = h.get(&team_name).copied().unwrap_or(0);
    //explication of the line below 
    //get return an option<&i32> , we do copy to get option<i32> without ref , after we do unwrap_or to set score to zero if h doesnt have an entry for the key

    for (key,value) in &h {
        println!("{key}:{value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue"); 

    let mut map = HashMap::new();
    map.insert(field_name,field_value);

    //field_name and field_value are moved so the map owns them because they dont have the copy trait like i32,f... 


    //Overwriting values in hashmap 

    let mut h = HashMap::new();
    h.insert(String::from("Blue"),10);
    h.insert(String::from("Blue"), 25); 

    //here we dont insert two keys "blue" , we update the value of the old key blue to 25 (a hashmap cant contain two or more same keys )

    h.entry(String::from("Blue")).or_insert(20); // entry return an enum that give us if the  key exists or not , so if the key exist we let the old value , otherwise we insert the new value  
    h.entry(String::from("Yellow")).or_insert(40);
    println!("{h:#?}"); 

    //exo1 
    {
     let mut v = vec![1,6,2,15];
     v.sort();
     let length = v.len(); 
     println!("the length is {}", length); 
     println!("the median is {}",v[length/2]);

     let mut h :HashMap<u8,u8> = HashMap::new(); 

     for i in v {
        let count = h.entry(i).or_insert(0);
        *count +=1 ; 
     }

     println!("map contains : {h:#?}");
    }

    }// all vectors, strings  are freed here
