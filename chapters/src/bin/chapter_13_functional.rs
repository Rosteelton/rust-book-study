use std::thread;

fn main() {
    //CLOSURES
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    //we need to explicitly move ownership to thread
    // (otherwise main thread can be completed earlier and there won't be list data anymore)

    //Implement one or more of the next traits:
    //FnOnce - basic move value out of the body. All closures implement this.
    //FnMut - don't move value out of the body but can mutate. Can be called more than once.
    //Fn - don't move, don't mutate

    let a: Vec<i32> = vec![-1, -2, 3, 4, 5]
        .into_iter()
        .filter(|a| a > &0)
        .map(|x| x + 1)
        .collect();

    println!("{:?}", a);
}
