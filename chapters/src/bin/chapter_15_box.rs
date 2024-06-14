use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

///Smart pointers implement *Deref* (enable struct behave like a reference)
///and *Drop* (what happens after outing of the scope) traits
/// PS. we can call `drop` function explicitly if we want to release memory earlier

///Box<T> - for storing value on the heap
/// 1. Size unknown at compile time + we want use in context where known size required
/// 2. Large data, we want transfer ownership without copying
/// 3. trait object

///Cons(i32, List) - can't do this.
/// RUST should know size of the structure.
/// Box it's only the pointer to the data (every next cons lay in the different space)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// with this trait we can use *
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:+?}", a);

    //dereference
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); //because we can't compare reference and value

    //Implicit Deref Coercions!!!
    fn hello(str: &str) {
        println! {"Hello {}", str}
    }
    let b = MyBox::new(String::from("RUST"));
    let c = &b; //it's equal to &**b
                // &b: &MyBox<String>
                // &*c: &String - can be mapped because of our implementation of Deref trait
                // &**c: *str (because in String defer method *str returned)
    hello(c);
}
