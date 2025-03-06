use back_of_house::Appetizer;
use back_of_house::Breakfast;// u just specify the path once , so u call Breakfast directly , it's like creating a symbolic link so the breakfast struct is valid in the scop 
use server::deliver_order;
//Note : still didnt understand the concept of using pub use
pub mod front_of_house;
mod back_of_house;
mod customer ;
mod server ; 