mod chapter_7_modules;

pub use crate::chapter_7_modules::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}