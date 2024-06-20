use std::ops::Add;
use std::slice;

///Unsafe:
/// - Dereference a raw pointer
/// - Call an unsafe function or method
/// - Access or modify a mutable static variable
/// - Implement an unsafe trait
/// - Access fields of a union
fn dereference_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn unsafe_function() {
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
        //(&mut values[..mid], &mut values[mid..]) - won't work due to double mutable borrows
    }
}

fn change_static_var() {
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

fn associated_types() {
    trait Iterator {
        type Item; //associated types
        fn next(&mut self) -> Option<Self::Item>;
    }

    trait Iterator2<T> {
        fn next(&mut self) -> Option<T>;
    }

    //the difference: in the 1st Iterator we can have only one implementation for struct
}

fn trait_ex() {
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn main() {
    dereference_raw_pointer();
    unsafe_function();
    change_static_var();

    //trait OutlinePrint: fmt::Display { - ensure that Display also be implemented for desired struct

    //struct Wrapper(Vec<String>); - newType pattern

    // <!>  type - panic! or continue = never return

    //Dynamically sized types:
    //RUST always should know the size of the data
    //That's wy it's impossible to us <str>, only reference to the string <&str>
    //If it's impossible to know the size of some data in compile time we need to use Pointer (ex: Box<>)


    //function in function syntax:
    //fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
}
