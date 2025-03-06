use adder::add; 

mod common; // bringin it 
#[test] 
fn it_adds_two() {
    common::setup();
    let result = add(1,2);
    assert_eq!(result,3);
} // we can test only the integration_tests using cargo test --test integration_tests.rs
  // we can also add a directory called common inside tests , that have the mod.rs file (old naming conventions of modules) , this is like a common functions to use that they dont need to be tested , because if we named common.rs it will appear also in the tests


  //Big note : this tests work only if u have the lib.rs , tsma chof kifah dir lib+main