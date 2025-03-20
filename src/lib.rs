#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

use alloc::{
    alloc::{alloc, dealloc},
    vec::Vec,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocator;
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { dealloc(ptr, layout) };
    }
}

pub fn foo() {
    let _v = Vec::<u32>::new();
}
