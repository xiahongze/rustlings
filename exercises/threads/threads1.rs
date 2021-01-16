// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let shared_status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    for i in 0..10 {
        let status = Arc::clone(&shared_status);
        thread::spawn(move || {
            let sleep_time = 100 - i * 10;
            thread::sleep(Duration::from_millis(sleep_time));
            let mut job_status = status.lock().unwrap();
            job_status.jobs_completed += 1;
            println!("job {} completed! Slept {}!", i, sleep_time);
        });
    }
    while shared_status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(200));
    }
}
