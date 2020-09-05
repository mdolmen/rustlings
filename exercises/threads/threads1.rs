// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut tmp = status_shared.jobs_completed.lock().unwrap();
            *tmp += 1;
        }
    });

    // Need another clone to increment the ref counter because previous one moved
    // into the thread
    let status_shared_clone = status.clone();
    // Need to deref "C-style" because of the mutex
    while *(status_shared_clone.jobs_completed.lock().unwrap()) < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
