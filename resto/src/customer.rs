pub fn eat_at_restaurant () {
    let mut meal = super::Breakfast::summer("Rye") ; // order a rye toast in summer
    // we are changing what toast we want so 
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);
    //we are not able to see or modify the fuit cuz it is private

    let order1 = super::Appetizer::Salad ;
    let order2 = super::Appetizer::Soup;

    //why super ? 
    //the parent module of customer is crate so we refer by super to crate , we can see that 
    // crate can access to breakfast cuz we declared the path to it in the first line 
    // so we can gain access to breakfast using super!  
}