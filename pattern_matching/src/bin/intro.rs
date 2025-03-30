fn main() {
    //if let statement 
    let favorite_color : Option<&str> = None ; 
    let if_sunday = false ; 
    let age : Result<u8,_> = "25".parse(); 

    if let Some(color) = favorite_color {
        println!("ur fav color is {}",color);
    }else if if_sunday {
        println!("sunday is a blue day");
    }else if let Ok(age) = age {
        if age > 20 {
            println!("bdit tekber déja {} ans",age);
        }else {
            println!("ykhi drari");
        }
    }
    //while let statement 
    let mut stack : Vec<i32> = Vec::new(); 
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(k) = stack.pop() {
        println!("{}",k);
    }

    println!("{:?}",stack);
}
