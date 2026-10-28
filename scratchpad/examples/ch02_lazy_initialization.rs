use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

fn keygen() -> u64 {
    static KEY : AtomicU64 = AtomicU64::new(0);

    let mut key = KEY.load(Relaxed);
    // previously 0, so generate new
    if key == 0 {
       key = 1024;
       KEY.store(key, Relaxed);
    }
    key
}

fn main(){
    keygen();    
}
