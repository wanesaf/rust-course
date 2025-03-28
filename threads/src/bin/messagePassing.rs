use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main () { 
//channel => genreal programming concept by which data is sent from one thread to another
let (tx,rx) = mpsc::channel(); // tx for transmitter , rx for receiver
//we can have multiple senders and only one receiver (mpsc)
let tx1 = tx.clone(); 
let handle = thread::spawn(move||{
    let vals = vec![
        String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
    ]; 
    
    for val in vals { 
        tx.send(val).unwrap();//val is moved
        thread::sleep(Duration::from_secs(1));
    }
}); 

handle.join().unwrap();

thread::spawn(move||{
    let vals = vec![
        String::from("more "),
            String::from("messages"),
            String::from("for"),
            String::from("u"),
    ]; 
    
    for val in vals { 
        tx1.send(val).unwrap();//val is moved
        thread::sleep(Duration::from_secs(1));
    }
}); 

//let received = rx.recv().unwrap();//this will block the main thread from execution and wait until we receive the val , if u want dont block main thread use try_recv     
// println!("value from the spawn thread is received : {}",received)

for received in rx {
    println!("Got it {}" , received);
}
}