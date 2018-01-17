use std::marker::PhantomData;
use std::ops::Deref;
use std::mem;
use std::ptr::{self, Unique};


pub struct Vec<T> {
    ptr: Unique<T>,
    cap: usize,
    len: usize
}

impl<T> Vec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "Can not handle ZSTs yet");
        Vec {
            ptr: Unique::empty(),
            len: 0,
            cap: 0
        }
    }

    fn oom(&self) {
        ::std::process::exit(-1);
    }

    fn grow(&mut self) {
        unsafe {
            
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();

            let (new_cap, ptr) = if self.cap == 0 {
                // pass in align to ensure alignment
                let ptr = heap::allocate(elem_size, align);
                (1, ptr)
            } else {
                // calculate the new capacity as being 2x the current cap
                let new_cap = self.cap * 2;
                // find the size of the old capacity
                let old_num_bytes = self.cap * elem_size;

                // check that new allocation doesn't exceed isize max
                // regardless of the capacity
                // we loose the ability to allocate 2/3rds of the
                // address
                assert!(
                    old_num_bytes 
                    <= 
                    // must be less than half the max size of isize
                    // as we are now doubling it and isize
                    // is the best
                    (::std::isize::MAX as usize) /2, "capacity of overflow");
                
                let new_num_bytes = old_num_bytes * 2;
                let ptr = heap::reallocate((self.ptr.as_ptr() as *mut _),
                                           old_num_bytes,
                                           new_num_bytes,
                                           align);
                (new_cap, ptr)
            };
            if ptr.is_null() { self.oom(); }
            self.ptr = Unique::new(ptr as *mut _).unwrap();
            self.cap = new_cap;
        }
    }


    pub fn push(&mut self, elem: T) {
        if self.len == self.cap { self.grow(); }

        unsafe {
            ptr::write(self.ptr.offset(self.len as isize), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.offset(self.len as isize)))
            }
        }
    }

}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let align = mem::align_of::<T>();
            let elem_size = mem::size_of::<T>();
            let num_bytes = elem_size * self.cap;
            unsafe {
                heap::deallocate(self.ptr.as_ptr() as *mut _, num_bytes, align);
            }
        }
    }
}