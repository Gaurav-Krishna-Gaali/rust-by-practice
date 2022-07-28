// Solution
// Add in lib.rs but i have not created a crate here to keep it simple
pub use crate::front_of_house::hosting; 

fn main() {
    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
     assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
