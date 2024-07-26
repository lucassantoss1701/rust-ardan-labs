use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

// fn poisoner() {
//     let mut lock = MY_SHARED.lock().unwrap();
//     *lock += 1;
//     panic!("The poisoner strikes")
// }

fn main() {

    let lock = MY_SHARED.lock().unwrap();
    std::mem::drop(lock);
    if let Ok(_lock) = MY_SHARED.try_lock() {
        println!("Got the lock")
    } else {
        println!("No lock")
    }
}
