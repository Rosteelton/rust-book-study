fn main() {
    //OWNERSHIP

    let s1 = String::from("hello");
    let s2 = s1;

    //not deep copy => after s1 data move to another pointer, s1 will be illuminated (no deep copies by default)

    //will be error:
    //println!("{}, world!", s1);


    let x = 5;
    let y = x; //that won't be illuminated because this is simple type on stack
    //it has copy trait implementation

    println!("x = {}, y = {}", x, y);

    //the same happens with functions

    fn get_ownership(x: String) {
        println!("{x}")
    }

    //there we can't use x anymore because drop function called after function finished its work

    //we can return data and assign to another variable:
    fn get_ownership_2(x: String) -> String {
        println!("{x}");
        x
    }

    let a1 = String::from("qwertyy");
    let a2 = get_ownership_2(a1); //a1 is not usable anymore

    //it's tedious
    //-------------
    //REFERENCE is the solution

    fn work_with_value(x: &String) -> &String { //& - use reference to the value without ownership
        //we can't modify value inside this function because we are not owner of it.
        println!("{x} inside function");
        x
    }

    let b1 = String::from("b1");
    let b2 = work_with_value(&b1); //BORROWING - we borrow value for the function work

    println!("{b1}");// in that case we can use b1

    //MUTABLE REFERENCE
    fn work_and_modify_value(x: &mut String) -> &String { //& - use reference to the value without ownership
        println!("{x} inside function");
        x.push_str("new part");
        x
    }

    let mut c1 = String::from("c1");
    let c2 = work_and_modify_value(&mut c1);

    println!("c2 = {c2}");

    let c3 = &mut c1;
    //let c4 = &mut c1; - we can't borrow mutable reference second time - it's guard from race condition.
    println!("{c3}");


    //but
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


    // We can't have:
    // - multiple mut reference
    // - mutable reference if we already have immutable once

    //-------------
    //SLICES

    let s: &str = "Hello, world!"; //&str = slice for String - immutable reference

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

}
