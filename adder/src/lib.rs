pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greetings (name : &str) -> String {
    println!("Hello {name}");
    format!("Hello {name}")
}

#[derive(Debug)]
pub struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn can_hold (&self , other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value : i32 
} 

impl Guess {
    fn new ( value:i32 ) -> Guess {
        if value < 1 {
            panic!("value must be greater than or equal one , got {value}");
        }
        else if value > 100 {
            panic!("value must be less or equal one hundered , got {value}"); 
        }
        else {
            Guess {value}
        }
    } 
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[ignore] // ignore the test
    fn panic() {
        panic!("make this test fail ");
    }
    #[test] 
    fn larger_than () {
        let rec1 = Rectangle {
            width : 30,
            height:30
        };
        let rec2 = Rectangle {
            width : 15 , 
            height : 30, 
        }; 
        assert!(rec1.can_hold(&rec2));
    }

    #[test]
    fn test_greetings () {
        let result = greetings("Carol"); 
        assert!(result.contains("Carol")
        ,"Greeting doesnt have the same name"
     );
    }
    
    #[test]
    #[should_panic]
    fn this_should_panic () {
        let guess = Guess::new(101);
    }

    #[test]
    #[should_panic(expected="less or equal one hundered")] // it panics only if the expected is a substring of the panic

    fn this_should_panic_using_expected () {
        let guess = Guess::new(101);
    }
    
    
}
 
// Notes : 
   // When we dont want to use any parallelism , we can run tests using 'cargo test -- --test-threads=1' , we use it when we dont want to have conflicts between tests
   // When a test passed we can show the output using , 'cargo test <function..> -- --show-output
   // u can test like functions that they contain a substring , example we have one , onehundred , twohundred : we can use test one to test the first two functions  
   // u can test only the ignored tests using cargo test -- --ignored
   // u can also test all the tests including ignored tests using cargo test -- --include-ignored

   //-------------------types of tests-------------------------- 
   //unit tests : test each unit in isolation , convention is to create test for each file in the src directory by creating a module named tests and annotating the module by [#cfg(test)]
    // the annotation mean that the test will run only when u do the cargo test aand not build , cfg stands for config so we add test to specify that this config runs only when we use test
   //integration tests : units tests can work fine , but when try to integrate them , there can be errors , so we do this integration test 
     //1. create a tests directory inside the src directory
     //2. 