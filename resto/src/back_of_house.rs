pub struct  Breakfast {
    pub toast : String, 
    seasonal_fruit :String // we dont want the client to choose the fruit so we make it private
}
pub enum Appetizer {Salad,Soup}
impl Breakfast { // we need the implemantion of the associated function , because seasonl_fruit is private , so there is no way to set the value of it without this function 
    pub fn summer (toast:&str) -> Breakfast {
        Breakfast {
            toast  : String :: from (toast),
            seasonal_fruit : String::from("peaches")  
        }
    }
}
pub fn fix_incorrect_order () {
       cook_order();
       super::deliver_order();//super refer to the parent module (lib in this case)
}
fn cook_order () {}