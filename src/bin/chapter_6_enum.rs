//ENUMS

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn is_quit(&self) -> bool {
        match self {
            Message::Quit => true,
            Message::Move { .. } => false,
            Message::Write(_) => false,
            Message::ChangeColor(_, _, _) => false,
        }
    }

    //same func - if let syntax
    fn is_quit_2(&self) -> bool {
        if let Message::Quit = self {
            true
        } else {
            false
        }
    }
}

fn main() {
    let a1 = Message::Quit;
    println!("is quit {}", Message::is_quit(&a1));
    println!("is quit 2 {}", Message::is_quit_2(&a1));
}
