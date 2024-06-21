use std::time::{Duration, SystemTime};
use std::thread;
use std::sync::{Arc, Mutex};

struct Task {
    run_at: SystemTime,
    task: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl Task {
    fn new<F>(run_at: SystemTime, task: F) -> Self
        where
            F: FnOnce() + Send + 'static,
    {
        Task {
            run_at,
            task: Some(Box::new(task)),
        }
    }
}

pub(crate) struct Scheduler {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl Scheduler {
    pub(crate) fn new() -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub(crate) fn schedule_task<F>(&mut self, run_at: SystemTime, task: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(Task::new(run_at, task));
    }

    pub(crate) fn start(&self) {
        let tasks = Arc::clone(&self.tasks);

        thread::spawn(move || loop {
            let now = SystemTime::now();
            let mut tasks_to_run = Vec::new();

            {
                let mut tasks = tasks.lock().unwrap();
                let mut i = 0;

                while i < tasks.len() {
                    if tasks[i].run_at <= now {
                        if let Some(task) = tasks[i].task.take() {
                            tasks_to_run.push(task);
                        }
                        tasks.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }

            for task in tasks_to_run {
                task();
            }

            thread::sleep(Duration::from_secs(1));
        });
    }
}
