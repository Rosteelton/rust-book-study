use std::fmt::Display;

fn main() {
    //Lifetimes

    //lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    println!("{}", longest("sdf", "asdff"));

    //!!!implicit lifetimes (lifetime elision) rules:
    //! 1. assigns a lifetime parameter to each parameter that’s a reference. fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    //! 2.if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32
    //! if there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

    let s: &'static str = "I have a static lifetime.";

    //example

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
        where
            T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}