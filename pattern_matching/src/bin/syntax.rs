struct Point {
    x:i32,
    y:i32
}

enum Message {
    Hello { id: i32 },
}
fn main () { 
    let x = 1 ; 
    match x {
        1|2 => println!("one or two"),
        3 => println!("three"),
        4..10 => println!("between for and ten "),
        _ => println!("not one")
    }

    let y = 'a' ; 
    match y {
        'a'..'z' => println!("lower case"), 
        'A'..'Z' => println!("upper"),
        _ => println!("other ASCII")
    }

    let y = Some(5) ; 
    let z = 10 ; 

    match y {
        Some(50) => println!("Got 50") , 
        Some(z) => println!("Got {z}") , 
        _ => println!("anything else")
    }

    let  a = Some(3) ; 
    let b =  10 ; 

    match a {
        Some(50) => println!("Got 50 ") , 
        Some(n) if n==b&&n%2!=0 => println!("Got odd number"),
        _ => println!("anything else")
    }

    //Destruction 
    let p : Point = Point { 
        x : 1, 
        y : 2
    }; 
    let Point {x,y} = p ; 
    println!("{x},{y}");  

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, //storing the value of id in id_variable in the same time of the test
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}