use std::cell::RefCell;
use std::rc::Rc;

use crate::List::{Cons, Nil};

///Rc<T> - if there will be many owners
/// ex: when we initiate smth, and don't know what part of our program will finish use this data earlier.
/// ! only for single thread
/// Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.
///
///RefCell<T> - need to avoid compiler restrictions about having more than one mutable references or both mutable and immutable references at the same time
/// but if the will have this will be checked in runtime, so you receive panic!
/// ! only for single thread
/// useful for mock objects
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

//if we do this:
//Cons(Rc<RefCell<i32>>, Rc<List>) - we can mutate value inside cons

trait Sender {
    fn send(&self, msg: &str) -> ();
}

struct MockSender {
    messages: RefCell<Vec<String>>,
}

impl Sender for MockSender {
    fn send(&self, msg: &str) -> () {
        // without ref cell we can't update internal state because argument &self in send function  - immutable one
        self.messages.borrow_mut().push(msg.to_string())
    }
}

fn main() {
    //with Box<T> we can't do this
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); //only increase reference counter, not a deep copy
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a)); //drop will decrease reference count auto

    let a1 = MockSender {
        messages: RefCell::new(vec![]),
    };

    a1.send("123");
    println!("messages = {:?}", &a1.messages.borrow())
    //so there is reference counter inside the structure

    //Если объекты ссылаются друг на друга через Rc<T> это может привести к циклическим зависимостям и утечке памяти.
    //Weak<T> используется для предотвращения. Не припятствует уничтожение объектов
}
