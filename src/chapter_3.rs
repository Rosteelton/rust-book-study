pub fn run() {
    /**
    CONST
    always immutable
    should be with explicit type
    should be evaluated in compile time
    **/
    const TWO_DAYS_SECOND: u32 = 2 * 24 * 60 * 60;

    //shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x + 1;
        println!("x = {x}");
    }

    println!("x = {x}");

    let x = 1_000_000;
    let x = 57u8;
    let x = 59.123;

    //tuple destructuring
    let tup = ("asd", 5, 5.0, ());
    let (a, b, c, d) = tup;

    //arrays are always fixed length
    let x = [1, 2, 3, 4, 5];

    for element in x {
        println!("{element}")
    }

    // [3, 3, 3, 3, 3]
    let x = [3; 5];

    fn func(x: i32) -> i32 {
        println!("The value of x is: {x}");
        x
    }

    let x = func(4);
    if x < 5 {
        println!("<5");
    } else if x > 5 {
        println!(">5");
    } else {
        println!("5");
    }

    //loop
    'loop1: loop {
        loop {
            break 'loop1;
        }
    }
}
