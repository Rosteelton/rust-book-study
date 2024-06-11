use std::collections::HashMap;

fn main() {
    //VECTOR
    let mut a: Vec<i32> = Vec::new();
    a.push(1);
    a.push(2);

    let b = vec![1, 2, 3];
    let b1 = b.get(2);
    let b2 = b[2];
    for i in &b {
        println!("{i}");
    }

    //STRING
    let mut c = "qwerty".to_string();
    let c1 = "qwerty";

    c.push_str(c1); //c1 = <&str> because we don't want to take ownership
    println!("{c1}"); //c1 is alive

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //we can't use index in string because String is Vec<u8>
    //and different letters have different amount of bytes that we need to store
    //so use <chars> method to iterate

    for i in s.chars() {
        println!("{i}");
    }
    println!("{s}");


    //HASHMAP
    let s0 = "qwerty";
    let mut h = HashMap::new();
    h.insert(s0.to_string(), 10);
    let result = h.get(s0).copied().unwrap_or(0);
    println!("{result}");

    h.entry(s0.to_string()).or_insert(50); //add if there i no value - fail
    h.entry("qwerty2".to_string()).or_insert(50); //add if there is no value - success
    println!("{:?}", h);

    //---------

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
