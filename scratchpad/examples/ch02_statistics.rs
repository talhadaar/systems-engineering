use std::{
    sync::atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
    thread,
    time::{Duration, Instant},
};

fn process_item() {
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let num_done = AtomicUsize::new(0);
    let total_time = AtomicU64::new(0);
    let max_time = AtomicU64::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    let start = Instant::now();
                    process_item();
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        // main thread
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));

            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                println!(
                    "{n}/100 done, {:?} avg, {:?} peak",
                    total_time / n as u32,
                    max_time,
                );
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done.");
}
