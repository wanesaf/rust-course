use std::sync::{Mutex,Arc}; 
use std::thread; 
fn main () {
    let m = Mutex::new(5) ; //chghol mutex hadi coffre tekder tefetho khetra at time 
    {
        let mut num = m.lock().unwrap() ; // lock function chghol bach te9der tefet7o lazem tkon endek autorisation 9bel li hiya lock , kon another thread 3ndo lock  rah y faili lcall wrah tsra panica
    //    let mut l = m.lock().unwrap(); // ucant do this , only one lock at scope , lazem 7ta y tunlocka by being dropped bach te9der tst3mel mutex khetrra wahdokhra
        *num = 6 ; 
   //     *l = 7 ; 
    }
    println!("m = {m:?}");


    {
        let counter = Arc::new(Mutex::new(5));//we dont use Rc because it's not a thread safe way , we use arc instead 
        let mut handles  = vec![] ; 
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move ||{
                let mut num =  counter.lock().unwrap(); 
               *num+=1;      
            }); 
            handles.push(handle);
        }

        for handler in handles {
            handler.join().unwrap();
        }

        println!("Result: {}",*counter.lock().unwrap());
    }
}