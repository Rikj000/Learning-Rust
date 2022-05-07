use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result { // --snip-- }
// fn function2() -> IoResult<()> { // --snip-- }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

// use crate::front_of_house::hosting; // Absolute path reference (Private)
// pub use crate::front_of_house::hosting; // Absolute path reference (Public)
// use self::front_of_house::hosting; // Relative path reference (Private)
pub use self::front_of_house::hosting; // Relative path reference (Public)

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}