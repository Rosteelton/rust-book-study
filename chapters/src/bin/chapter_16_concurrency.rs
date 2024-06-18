use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;


fn basic_thread_demo() {
    //the way to run the thread (1 to 1 model to OS thread):
    let a = thread::spawn(|| {
        for i in 1..5 {
            println!("{} from spawned thread", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    //when main thread stop to work the program will be shutdown
    for i in 1..5 {
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(500));
    }

    a.join().unwrap(); //now the main thread will wait until a thread A finish work
}

fn passing_var_inside_thread_demo() {
    let b = vec![1,2,3];

    let c = thread::spawn(move || { //with <move> we are able to send ownership
        println!("{:?}", b)
    });

    //can't use b anymore

    c.join().unwrap();
}

///Channels - for sharing data between thread, receiver + transmitter - single ownership
fn channel_demo() {
    let (tx, rc) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let arr = vec![1,2,3,4,5];
        for i in arr {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        let arr = vec![6,7,8,9,10];
        for i in arr {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for el in rc {
        println!("{}", el); //we see all elements because main thread will wait until all tx will be dropped.
    };
}

///Mutex - use data between threads - like multiple ownership
///+ Arc (atomic references counter)- concurrent version of Rc
fn mutex_demo() {
    let mut threads = vec![];
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let mutex = Arc::clone(&counter);
        let thread = thread::spawn(move || { //without Arc we can't do this, because we sent mutex ownership in first loop
           let mut val = mutex.lock().unwrap();
            *val += 1; //* is possible because MutexGuard has Deref instance
        });
        threads.push(thread);
    }


    for t in threads {
        t.join().unwrap(); // wait until all spawned threads finish work
    }

    println!("Total count = {}", *counter.lock().unwrap())
}

fn main() {
    basic_thread_demo();

    passing_var_inside_thread_demo();

    channel_demo();

    mutex_demo();

    //Send marker - ownership of values can be transferred between threads.
    //Sync marker - type can be referenced from multiple threads.
    //Any type T is Sync if &T (an immutable reference to T) is Send.
}