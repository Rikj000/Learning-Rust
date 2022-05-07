mod front_of_house;

pub use crate::front_of_house::hosting; // Absolute path reference (Public)

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}