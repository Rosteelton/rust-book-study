//STRUCT

struct Color(i32, i32, i32); //named tuple

struct NoField;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    is_active: bool,
}

impl User {
    //constructor
    fn init_user(name: String, age: u32) -> Self {
        Self {
            is_active: true,
            name,
            age,
        }
    }

    //method for some User instance
    fn years_to_50(&self) -> u32 {
        if self.age > 50 {
            0
        } else {
            50 - self.age
        }
    }
}

fn main() {
    let user1 = User::init_user(String::from("qwerty"), 32);
    let user2 = User {
        is_active: true,
        ..user1 //it means, that other fields would be the same as user1
    };

    println!("{:#?}", user2);

    //we can use user 1 after this point!!! because we moved some heap values to another object
    //println!("{:#?}", user1); - not compile

    println!("Years to 50: {}", user2.years_to_50())
}
