use std::sync::Mutex;
use std::thread;

static COUNTER: Mutex<u64> = Mutex::new(0);

pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    {
        let mut counter = COUNTER.lock().unwrap();
        *counter = 0;
    }

    let mut handles = Vec::new();
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                let mut counter = COUNTER.lock().unwrap();
                *counter += 1;
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }
    *COUNTER.lock().unwrap()
}

pub fn reset_counter() {
    let mut counter = COUNTER.lock().unwrap();
    *counter = 0;
}
