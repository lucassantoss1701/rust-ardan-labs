fn allocate_memory_with_libc(){
    unsafe{
        let my_num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if my_num.is_null(){
            panic!("failed to allocate memory");
        }

        // Set the allocated variable - deference the pointer and set to 42
        *my_num = 42;
        assert_eq!(42, *my_num);

        // Free the memory with libc - this is NOT automatic
        libc::free(my_num as *mut libc::c_void);
    }
}

fn allocate_memory_with_rust(){
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        // Set the allocated variable - deference the pointer and set to 42
        *ptr = 42;
        assert_eq!(42, *ptr);

        // Free the memory - this is NOT automatic
        dealloc(ptr, layout);

    }
}


fn main() {
    allocate_memory_with_rust();
    allocate_memory_with_libc();
}
