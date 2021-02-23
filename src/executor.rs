use std::sync::{Arc, Mutex, RwLock};
use std::collections::VecDeque;
use std::sync::mpsc::{Sender, Receiver};
use crate::task::{Task, TaskState};
use crate::policy::Policy;


pub struct Executor {
    pub policy: Arc<Mutex<Policy>>,
    pub waiting: RwLock<VecDeque<Box<Task>>>,
    pub running: RwLock<VecDeque<Box<Task>>>,
}

impl Executor {
    pub fn new(policy: Arc<Mutex<Policy>>) -> Executor {
        Executor {
            policy,
            waiting: RwLock::new(VecDeque::new()),
            running: RwLock::new(VecDeque::new()),
        }
    }

    pub fn add_task(&self, task: Box<Task>) {
        self.waiting.write().unwrap().push_back(task);
    }

    pub fn run(&self) {
        loop {
            if self.waiting.read().unwrap().len() > 0 || self.running.write().unwrap().len() < 64 {
                let task = self.waiting.write().unwrap().pop_front();
                if let Some(mut task) = task {
                    self.running.write().unwrap().push_back(task);
                }

            }
            let task = self.running.write().unwrap().pop_front();

            if let Some(mut task) = task {
                if task.run().0 == TaskState::COMPLETED {
                    println!("{}", "task finish.");
                    task.finish();
                } else {
                    println!("{}", "check server overhead.");
                    self.running.write().unwrap().push_back(task);
                }
            }
        }
    }
}

unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}
