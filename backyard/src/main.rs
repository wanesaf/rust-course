pub mod garden; // tell the compiler to include the code found in garden.rs , adding pub cuz the code in garden is private by default
use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {} ; 
    println!("I'm growing {plant:#?}");
}
