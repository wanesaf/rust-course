#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USstate),
}
#[derive(Debug)]
enum USstate {
    Alabama,
    Alaska,
}
impl Message {
    fn call(&self) {}
}

fn main() {
    let home = IpAddressKind::V4(127, 0, 0, 1);
    let loopback = IpAddressKind::V6(String::from("::1"));

    println!("{home:#?},{loopback:#?}");

    let m = Message::Write(String::from("Hello World"));
    m.call();

    //using option enum provided by standard library cuz null doesnt exist in rust
    {
        let some_number = Some(5);
        let some_char = Some('a');
        let absent_number: Option<i32> = None;

        println!("{absent_number:#?}");
    }

    let coin_choosen = Coin::Quarter(USstate::Alabama);
    let x: u8 = value_in_cents(&coin_choosen);
    println!("{x}");

    println!("{coin_choosen:?}"); // lessgo borrow is working mman

    let value = Some(1) ; 
    println!("{:?}",plus_one(value));
    let no_value : Option<u8>= None ; 
    println!("{:#?}",plus_one(None));

    //diff between match and if statement , match ensure that all possiblitites must be covered , if doesnt do this , so we called match by "exhaustive match" 

    let roll = 9 ; 

    match roll {
        3 => add_fancy_cat(), 
        7 => remove_fancy_cat(), 
        other => move_player(other) // if u want to not bind the value of other (u dont want to use it like we have done) u can use the (_)  
    }

    //using if let 
    let mut count = 0;
    if let  Coin::Quarter(state) = coin_choosen { // if let cghol sigha mokhtasara tae match djat bach chghol match9ach dir lexhaustive check tae ki t ignori other values hada makan 
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
        println!("{count}");
    }
}

fn route(ip_addr: IpAddressKind) {}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

fn plus_one(option: Option<u8>) -> Option<u8> {
    match option {
        None => {
            println!("There is no value ");
            None
        },
        Some(i) => {
            println!("The value passed is {i}");
            Some(i+1)
        }
}
}

fn add_fancy_cat() {} 
fn remove_fancy_cat() {} 
fn move_player(num_spaces : u8) {}
