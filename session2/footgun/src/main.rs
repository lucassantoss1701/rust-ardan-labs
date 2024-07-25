use std::sync::atomic::AtomicI32;

static mut COUNTER: i32 = 0;
// static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000{
        let handle = std::thread::spawn(||{
            for _ in 0..1_1000{
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h|{
       h.join().unwrap()
    });

    unsafe {
        println!("{COUNTER}")
    }
}
