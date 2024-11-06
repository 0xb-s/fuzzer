use std::alloc::{GlobalAlloc, Layout, System};

pub struct CustomAllocator;

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //todo
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //todo
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: CustomAllocator = CustomAllocator;
