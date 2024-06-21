mod task_scheduler;
use std::thread;
use task_scheduler::Scheduler;
use std::time::{SystemTime, Duration};

fn main() {
    let mut scheduler = Scheduler::new();

    let now = SystemTime::now();

    scheduler.schedule_task(now + Duration::new(5, 0), || {
        println!("Task 1 executed!");
    });

    scheduler.schedule_task(now + Duration::new(10, 0), || {
        println!("Task 2 executed!");
    });

    scheduler.schedule_task(now + Duration::new(15, 0), || {
        println!("Task 3 executed!");
    });

    println!("Starting scheduler...");
    scheduler.start();

    // To keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
