use std::{
    sync::atomic::{AtomicU32, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    let num_done = AtomicU32::new(0);

    thread::scope(|s| {
        // 4 bkg workers
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    // consume some taym
                    num_done.fetch_add(1, Relaxed);
                }
            });
        }
        // main thread
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done.");
}
