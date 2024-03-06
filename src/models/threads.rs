use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

use std::thread;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver
                .lock()
                .expect("ERROR::WORKER FAILED TO ACQUIRE LOCK ON RECEIVER")
                .recv()
            {
                job();
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn set_size(&mut self, size: usize) {
        assert!(size > 0);

        let (new_sender, new_receiver) = mpsc::channel();
        let new_receiver = Arc::new(Mutex::new(new_receiver));

        for worker in &mut self.workers {
            self.sender.send(Box::new(|| {})).unwrap();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        self.sender = new_sender;
        self.workers.clear();

        for id in 0..size {
            self.workers.push(Worker::new(id, Arc::clone(&new_receiver)));
        }
    }

    pub fn shutdown(&mut self) {
        for _ in &self.workers {
            self.sender.send(Box::new(|| {})).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn thread_id() -> u64 {
    let mut string = format!("{:?}", thread::current().id());
    string.replace_range(0..9, "");
    string.pop();
    string.parse().unwrap()
}