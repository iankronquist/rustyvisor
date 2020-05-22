// FIXME This is the simplest most naive implementation of an allocator. I
// basically envisioned it in C in my head and translated it on the fly. I hope
// to revisit this someday when I know the language better, but in the meantime
// don't take this as an example for anything ever.

#![feature(allocator)]
#![feature(asm)]
#![feature(const_fn)]

#![allocator]
#![allow(not_unsafe_ptr_arg_deref)]
#![allow(unknown_lints)]
#![no_std]

use core::fmt::{Write};
use core::ptr;
use core::mem;

use self::spin::Mutex;

pub mod serial_logger_lite;

const SPLIT_FUDGE_FACTOR: usize = 32;

#[derive(Debug)]
#[repr(C)]
struct Region {
    size: usize,
    prev: Option<*mut Region>,
    next: Option<*mut Region>,
}

pub struct Allocator {
    heap_start: *mut u8,
    heap_size: usize,
    free_list: Option<*mut Region>,
}

static mut ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator {
    heap_start: 0 as *mut u8,
    heap_size: 0,
    free_list: None,
});

pub fn init(size: u64, raw_bytes: *mut u8) {
    static mut CALLED: bool = false;

    unsafe {
        assert!(!CALLED);
        init_allocator(&mut ALLOCATOR.lock(), size as usize, raw_bytes);
        CALLED = true;
    }
}

fn init_allocator(allocator: &mut Allocator, size: usize, raw_bytes: *mut u8) {

    assert!(!raw_bytes.is_null());

    let mut region = raw_bytes as *mut Region;
    unsafe {
        (*region).size = size;
        (*region).next = None;
        (*region).prev = None;
    }
    allocator.heap_size = size;
    allocator.heap_start = raw_bytes;
    allocator.free_list = Some(raw_bytes as *mut Region);
}

// Serial logger & allocator must be initialized before calling this function.
pub fn debug_allocator() {
    unsafe {
        let allocator = ALLOCATOR.lock();
        let mut current_region = allocator.free_list;
        let mut s: serial_logger_lite::SerialLogger = Default::default();

        while let Some(region) = current_region {
            let _ = write!(&mut s, "Region {:?} {:?}\n", region, region);
            current_region = (*region).next;
        }
    }
}

impl Allocator {
    fn free(&mut self, bytes: *mut u8) {

        let mut region: *mut Region;

        if bytes.is_null() {
            return;
        }

        unsafe {
            let offset = -(mem::size_of::<Region>() as isize);
            region = bytes.offset(offset) as *mut Region;

            assert!(region as *mut u8 >= self.heap_start);

            assert!(!region.is_null());

            (*region).prev = None;
            (*region).next = self.free_list;
        }

        self.free_list = Some(region);
    }

    fn alloc(&mut self, size: usize) -> *mut u8 {

        let region_offset = mem::size_of::<Region>() as isize;
        let real_size = size + mem::size_of::<Region>();
        let mut current_region = self.free_list;

        while let Some(region) = current_region {

            unsafe {
                if (*region).size >= real_size {
                    let new_next: Option<*mut Region>;
                    let new_prev: Option<*mut Region>;
                    let split = (*region).size - real_size;
                    if split > (region_offset as usize) + SPLIT_FUDGE_FACTOR {
                        let split_region =
                            (region as *mut u8)
                                .offset((size as isize) +
                                        region_offset) as *mut Region;
                        (*region).size -= size + region_offset as usize;
                        new_next = Some(split_region);
                        new_prev = Some(split_region);
                    } else {
                        new_next = (*region).next;
                        new_prev = (*region).prev;
                    }
                    match (*region).prev {
                        None => {}
                        Some(prev) => (*prev).next = new_next,
                    }
                    match (*region).next {
                        None => {}
                        Some(next) => (*next).prev = new_prev,
                    }
                    if new_next == None && new_prev == None {
                        self.free_list = None;
                    }
                    return (region as *mut u8).offset(region_offset);
                }
                current_region = (*region).next;
            }
        }

        ptr::null::<u8>() as *mut u8
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { ALLOCATOR.lock().alloc(size) }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe {
        ALLOCATOR.lock().free(ptr);
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    old_size: usize,
                                    new_size: usize,
                                    align: usize)
                                    -> *mut u8 {
    use core::cmp;
    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(old_size, new_size)) };
    __rust_deallocate(ptr, old_size, align);
    new_ptr
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(_ptr: *mut u8,
                                            old_size: usize,
                                            _size: usize,
                                            _align: usize)
                                            -> usize {
    old_size
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}


#[cfg(test)]
mod tests {
    use core::mem;

    use super::Allocator;
    use super::Region;
    use super::init_allocator;

    #[test]
    fn test_init_allocator() {

        let mut al = Allocator {
            heap_size: 0,
            heap_start: 0 as *mut u8,
            free_list: None,
        };

        let mut memory: [u8; 100] = [0; 100];
        let pmemory: *mut u8 = memory.as_mut_ptr();
        init_allocator(&mut al, mem::size_of::<[u8; 100]>(), pmemory);
        assert_eq!(al.heap_size, 100);
        assert_eq!(al.heap_start, pmemory);

        match al.free_list {
            None => assert!(false),
            Some(fl) => {
                assert!(!fl.is_null());
                unsafe {
                    assert_eq!((*fl).size, 100);
                    assert_eq!((*fl).next, None);
                };
            }
        }
    }

    #[test]
    fn test_alloc_and_free() {

        let mut al = Allocator {
            heap_size: 0,
            heap_start: 0 as *mut u8,
            free_list: None,
        };

        let mut memory: [u8; 1000] = [0; 1000];
        let pmemory: *mut u8 = memory.as_mut_ptr();
        init_allocator(&mut al, mem::size_of::<[u8; 1000]>(), pmemory);

        let m0_size = 10000 - mem::size_of::<Region>();
        let m0 = al.alloc(m0_size);
        assert!(m0.is_null());


        let m1_size = 1000 - mem::size_of::<Region>();
        let m1 = al.alloc(m1_size);
        assert!(!m1.is_null());

        assert_eq!(al.free_list, None);
        al.free(m1);
        assert_ne!(al.free_list, None);

        let m2_size = 100;
        let m2 = al.alloc(m2_size);
        assert!(!m2.is_null());

        let m3_size = 10;
        let m3 = al.alloc(m3_size);
        assert!(!m3.is_null());

        let m4_size = 1;
        let m4 = al.alloc(m4_size);
        assert!(!m4.is_null());

        assert_ne!(al.free_list, None);
        al.free(m3);

        assert_ne!(al.free_list, None);
        al.free(m2);

        assert_ne!(al.free_list, None);
        al.free(m4);
    }
}
