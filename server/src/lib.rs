use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().map(|s| s.send(job).unwrap());
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut vec: Vec<Worker> = Vec::with_capacity(size);
        let (tx, rc): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(rc));
        //arc - share ownership between threads
        //mutex - manipulate via lock

        for i in 0..size {
            vec.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            sender: Some(tx),
            workers: vec,
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            worker.thr.take().map(|s| s.join().unwrap());
        }
    }
}

pub struct Worker {
    id: usize,
    thr: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, rc: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thr: Some(thread::spawn(move || loop {
                let message = rc.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            })),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
