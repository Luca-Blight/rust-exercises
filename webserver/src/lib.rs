pub struct ThreadPool;

use std::thread::JoinHandle;
use std::thread;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        thread::spawn(f)
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
