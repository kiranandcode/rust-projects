use std::marker::PhantomData;
use std::mem::align_of;

pub struct RefWithFlag<'a, T:'a> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T> // occupies no space but helps type checking
}

impl<'a, T: 'a> RefWithFlag<'a, T> {
    pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
        // must be even pointer to perform this bit fiddling trick
            assert!(align_of::<T>() % 2 == 0);
        RefWithFlag {
            ptr_and_bit: ptr as *const T as usize | flag as usize,
            behaves_like: PhantomData
        }
    }

    pub fn get_ref(&self) -> &'a T {
        unsafe {
            // remove lowermost bit
            let ptr = (self.ptr_and_bit & !1) as *const T;
            // cant create a ref from a pointer - must deref and then ref
            &*ptr
        }
    }

    pub fn get_flag(&self) -> bool {
        self.ptr_and_bit & 1 != 0
    }
}