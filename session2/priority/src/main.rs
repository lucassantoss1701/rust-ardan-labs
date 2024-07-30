use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;
use thread_priority::*;


//DEPENDS ON THE OS
static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MEDIUM_COUNT: AtomicI32 = AtomicI32::new(0);
static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);

fn low_prio(){
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop{
        LOW_COUNT.fetch_add(1, Ordering::Relaxed);
        std::thread::yield_now()
    }
}

fn regular_prio(){

    // set_current_thread_priority(ThreadPriority::Max).unwrap();
    loop{
        MEDIUM_COUNT.fetch_add(1, Ordering::Relaxed);
        std::thread::yield_now()
    }
}

fn main() {
    std::thread::spawn(low_prio);
    std::thread::spawn(regular_prio);
    std::thread::sleep(Duration::from_secs(5));
    println!("Low: {}", LOW_COUNT.load(Ordering::Relaxed));
    println!("Medium: {}", MEDIUM_COUNT.load(Ordering::Relaxed));

}
