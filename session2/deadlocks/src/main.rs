use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("The poisoner strikes");
}

fn main() {
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");

    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering data...");
        poisoned.into_inner()
    });

    println!("{recovered_data:?}")
}
