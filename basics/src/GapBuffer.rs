use std;
use std::ops::Range;

pub struct GapBuffer<T> {
    storage: Vec<T>,
    // range of values which are unsized
    gap: Range<usize>
}

impl<T> GapBuffer<T> {
    pub fn new() -> GapBuffer<T> {
        GapBuffer {
            storage: Vec::new(), gap: 0..0
        }
    }

    /// Return the number of elements this gapbuffer
    /// can hold without reallocation
    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    /// Return the number of elements this GapBuffer currently holds
    pub fn len(&self) -> usize {
        // total capacity - minus the gap size
        self.capacity() - self.gap.len()
    }

    /// Return the current insertion position
    pub fn position(&self) -> usize {
        self.gap.start
    }

    /// Returns a pointer to the indexth element of the storage
    /// irrespective of the gap
    ///
    /// Safety: 'index' must be a valid index into self.storage
    unsafe fn space(&self, index: usize) -> *const T {
        self.storage.as_ptr().offset(index as isize)
    }

    /// Return a mutable pointer into the indexth element of the
    /// underlying storage
    ///
    /// Safety: 'index' must be a valid index into self.storage
    unsafe fn space_mut(&mut self, index: usize) -> *mut T {
        self.storage.as_mut_ptr().offset(index as isize)
    }


    /// Return the offset in the buffer of the 'index'th
    /// element taking into account the gap
    /// Does not check whether index in range, but never
    /// returns an index in the gap
    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            // the gap represents a gap in the indecies, so adding the length will return the index you want
            index + self.gap.len()
        }
    }

    /// Return a reference to the indexth element
    /// or none if index is out of bounds
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = self.index_to_raw(index);
        if raw < self.capacity() {
            unsafe {
                Some(&*self.space(raw))
            }
        } else {
            None
        }
    }

    /// Set the current insertion position to pos
    // if pos is out of bounds panic
    pub fn set_position(&mut self, pos:usize) {
        if pos > self.len() {
            panic!("index {} out of range for GapBuffer", pos);
        }

        unsafe {
            let gap = self.gap.clone();
            if pos > gap.start {
 
                let distance = pos - gap.start;

                // move from the end of gap into the front of gap distance
                // move gap right by moving items to right of gap to left 
                std::ptr::copy(self.space(gap.end), self.space_mut(gap.start), distance);
                // gap ends right before pos
            } else if pos < gap.start {
                // move gap left by shifting  elements befor the gap to after it
                // means that our position now lies after the gap
                // i.e
                //               ...10, 12, ........,13, 14, 15...
                //                      
                //                         <- gap ->
                // [############### ### ###_________### ### ### ####]
                //                                       pos
                //               ...10, 12, ........,13, | , 15...
                //                                       | 
                //                         <- gap ->     v
                // [############### ### ###_________### ### ### ####]

                // so we need to           <---------(gap.len())  <-------- distance to move = pos - start of gap
                //                                 <-move      ->       <-------- (size to move = pos -  end of gap)
                //               ...10, 12, ........,13, 14, 15...
                //                                        
                //                         <- gap ->     
                // [############### ### ###_________### ### ### ####]
 
                // so we need to          start         pos
                //                         |-------------|
                //                            distance
                //               ...10, 12, ........,13, 14, 15...
                //                                        
                //                         <- gap ->     
                // [############### ### ###_________### ### ### ####]
 
                //                        __copy to__    pos
                // so we need to          |          end  |        end + distance
                //                        |          |----v--------|
                //                        v              distance
                //               ...10, 12, ........,13, 14, 15...
                //                                        
                //                         <- gap ->     
                // [############### ### ###_________### ### ### ####]
                //                     old gap   pos     new gap  
                //                        |      |         |
                //                        v      v         v
                //               ...10, 12, 13, 14, 15, 16, ........
                //                                        
                //                                 <- gap -> 
                // [############### ### ### ### ###_________### ####]
 



                let distance = gap.start - pos;
                std::ptr::copy(self.space(pos), self.space_mut(gap.end - distance), distance);
                // gap now right after pos
            }

            self.gap = pos..pos + gap.len();
        }
    }

    /// Insert `elt` at the current insertion position
    /// and leave the insertion position after it
    pub fn insert(&mut self, elt: T) {
        if self.gap.len() == 0 {
            self.enlarge_gap();
        }
        unsafe {
            let index = self.gap.start;
            std::ptr::write(self.space_mut(index), elt);
        }

        self.gap.start += 1;
    }

    pub fn insert_iter<I>(&mut self, iterable:I) where I: IntoIterator<Item=T> {
        for item in iterable {
            self.insert(item)
        }
    }

    /// Remove the element just after the insertion position 
    /// and return it or return 'None' if the insertion position 
    /// is at the end of the GapBuffer
    pub fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }

        let element = unsafe {
            std::ptr::read(self.space(self.gap.end))
        };

        self.gap.end += 1;
        Some(element)
    }


    /// Double the capacity of self.storage
    fn enlarge_gap(&mut self)  {
        let mut new_capacity = self.capacity() * 2;
        if new_capacity == 0 {
            new_capacity = 4;
        }

        let mut new = Vec::with_capacity(new_capacity);
        let after_gap = self.capacity() - self.gap.end;
        let new_gap = self.gap.start..new.capacity()-after_gap;
        unsafe {
            std::ptr::copy_nonoverlapping(self.space(0), new.as_mut_ptr(), self.gap.start);
            let new_gap_end = new.as_mut_ptr().offset(new_gap.end as isize);

            std::ptr::copy_nonoverlapping(self.space(self.gap.end), new_gap_end, after_gap);
        }

        self.storage = new;
        self.gap = new_gap;
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.gap.start {
                std::ptr::drop_in_place(self.space_mut(i));
            }
            for i in self.gap.end .. self.capacity() {
                std::ptr::drop_in_place(self.space_mut(i));
            }
        }
    }
}