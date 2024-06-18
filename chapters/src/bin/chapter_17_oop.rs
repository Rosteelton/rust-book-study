/// There is no inheritance of objects in RUST
/// There is bounded parametric polymorphism via traits
/// There is incapsulation over pub modifier
/// There are objects via <Struct> and methods for them in <impl> section.

pub trait Draw {
    fn draw(&self);
}

///dyn Draw - any struct that implements Draw - trait object
/// dynamic dispatch => additional perf cost and no compiler optimizations
///
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run_draw(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Point {
    x: u32,
    y: u32,
}

pub struct Button {
    place_holder: String,
}

impl Draw for Button {
    fn draw(&self) {
        todo!()
    }
}

impl Draw for Point {
    fn draw(&self) {
        todo!()
    }
}

fn main() {}
