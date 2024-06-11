use std::cmp::Ordering;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Something<T: PartialOrd> {
    x: T,
}

//or just use #[derive(PartialOrd, PartialEq)] on struct
impl<T: PartialOrd> PartialEq for Something<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}
impl<T: PartialOrd> PartialOrd for Something<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

//общая функция, использующая трейт в параметре
pub fn some_func(item1: &impl PartialOrd) {}

//using trait bounds:
pub fn is_greater_2<T: PartialOrd>(item1: &T, item2: &T) -> bool {
    item1.gt(item2)
}

//<T: Summary + Display> - for several traits or:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    32
}

fn main() {
    fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for el in list {
            if el > largest {
                largest = el;
            }
        }
        largest
    }

    println!("The largest: {}", find_largest(&[1, 2, 3, 4, 5]));
    println!("The largest: {}", find_largest(&['a', 'b', 'c', 'd', 'e']));
    println!(
        "The largest: {:?}",
        find_largest(&[
            Something { x: 1 },
            Something { x: 2 },
            Something { x: 3 },
            Something { x: 4 },
            Something { x: 5 }
        ])
    );

    //monomorphization - after compilation there would be structures only for [i32] and [char] types
    // => no runtime perf costs

    //coherence - orphan rule - мы можем реализовывать трейты для структур или энумов, которые вы сами определили,
    // или реализовывать наши собственные трейты для любых типов. Но вы не можете реализовать чужие трейты для чужих типов.

    //в trait как и в scala может быть дефолтная реализация методов
}
