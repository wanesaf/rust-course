#[derive(PartialEq ,Debug)]
struct Shoe {
  size : u32,
  style : String 
}


fn main() {
    let v1 = vec![1,2,3]; 
    let mut v1_iter = v1.iter();
    println!("{:?}",v1_iter);

    assert_eq!(v1_iter.next(),Some(&1));

    let total : i32 = v1_iter.sum();
    println!("{}",total); 

    //println!("{}",v1_iter);//sum take ownership of the iterator , u cant call it 

    let new_vector : Vec<_> = v1.iter().map(|x| x+1).collect(); // tconsommer l iterator wt7ot result tae lclosure dans un vect

}


fn shoes_in_size (shoes : Vec <Shoe> , shoe_size : u32) -> Vec<Shoe> {
shoes.into_iter().filter(|s| s.size == shoe_size).collect() // into_iter takes ownership of the vector , iter() only dont do that
}