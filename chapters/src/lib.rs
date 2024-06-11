mod chapter_7_modules;
pub use crate::chapter_7_modules::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//duplicated for tests reason
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn is_quit(&self) -> bool {
        match self {
            Message::Quit => true,
            Message::Move { .. } => false,
            Message::Write(_) => false,
            Message::ChangeColor(_, _, _) => false,
        }
    }
}
