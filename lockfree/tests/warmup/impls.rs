use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn parallel_sum(nums: &[u64]) -> u64 {
    thread::scope(|s| {
        let chunk_size = nums.len().div_ceil(4).max(1);

        // Collect handles
        let handles: Vec<_> = nums
            .chunks(chunk_size)
            .map(|c| s.spawn(move || c.iter().sum::<u64>()))
            .collect();

        // Resolve handles
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

// Using Arc<Mutex<u64>>
pub fn shared_counter_v1(t: u64, n: u64) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));

    let handles: Vec<_> = (0..t)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..n {
                    *c.lock().unwrap() += 1;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    *counter.lock().unwrap()
}
