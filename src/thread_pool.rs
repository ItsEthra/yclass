#![allow(dead_code)]

use parking_lot::Mutex;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{park, spawn, JoinHandle},
};

type TaskPool = Arc<Mutex<Vec<Box<dyn Task>>>>;

pub struct ThreadPool {
    threads: Vec<ThreadHandle>,
    tasks: TaskPool,
    limit: usize,
}

pub struct ThreadHandle {
    handle: JoinHandle<()>,
    parked: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(limit: usize) -> Self {
        let tasks: TaskPool = TaskPool::default();
        let mut threads = vec![];

        for _ in 0..limit {
            let parked: Arc<AtomicBool> = Arc::default();
            let handle = spawn({
                let parked = parked.clone();
                let tasks = tasks.clone();

                move || loop {
                    let mut lock = tasks.lock();
                    if let Some(mut task) = lock.pop() {
                        drop(lock);

                        task.execute();
                        continue;
                    }
                    drop(lock);

                    parked.store(true, Ordering::SeqCst);
                    park();
                }
            });

            threads.push(ThreadHandle { handle, parked });
        }

        Self {
            threads,
            tasks,
            limit,
        }
    }

    pub fn spawn(&self, task: impl Task) {
        self.tasks.clone().lock().push(Box::new(task));

        for thread in self.threads.iter() {
            if thread.parked.load(Ordering::SeqCst) {
                thread.handle.thread().unpark();
            }
        }
    }

    pub fn unpark_all(&self) {
        for thread in self.threads.iter() {
            if thread.parked.load(Ordering::SeqCst) {
                thread.handle.thread().unpark();
            }
        }
    }
}

pub trait Task: Send + Sync + 'static {
    fn execute(&mut self);
}

impl<T: FnMut() + Send + Sync + 'static> Task for T {
    #[inline]
    fn execute(&mut self) {
        self();
    }
}
