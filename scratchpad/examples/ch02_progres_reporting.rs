use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    // some stuff
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // Bg thread processing 100 items
        s.spawn(|| {
            for i in 0..100 {
                // some process
                num_done.store(i + 1, Relaxed);
            }
        });

        // main thread checks status every 1s
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Ongoing... {n}/100 done.");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
