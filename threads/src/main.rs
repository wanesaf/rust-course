use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));//stop the execution of the thread for short duration => allow main thread to run 
        }
    });

    //handle.join().unwrap(); calling handle here will affect the programm , the spawn thread must complete the loop then the main thread y9la3

    for i in 1..5 {
        println!("hi {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));//same shit 
    } // when the main thread complete , the spawned thread will shut down 

    handle.join().unwrap();//calling join on the handle blocks the thread currently running until the thread represented by the handle terminates, meaning we block main thread from exiting so the spawn thread complete its loop . 

    {
        let v = vec![1,2,3]; 

        let handle = thread::spawn( move ||{
            println!("Here's a vector : {v:?}");
        }); // move enforce the closures to take ownership of the values it's using , puisque lokan tborrowi mat9derch ta3ref est ce que refs mazalhom valide wela non capable ykon tdroppaw wthread mazal makmel 

        handle.join().unwrap();
    }

}
