#![no_std]
#[cfg(test)]
mod tests;


use core::{
    ptr::{
        NonNull,
        null_mut,
    },
    marker::PhantomData,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct LinkedListAnchor{
    prev: Option<isize>,
    next: Option<isize>,
}

#[derive(Debug)]
pub struct LinkedList<const OFFSET:usize, T> {
    base: *mut u8,
    head: Option<isize>,
    tail: Option<isize>,
    len: usize,
    phantom: PhantomData<T>,
}


impl<const OFFSET:usize, T> LinkedList<OFFSET, T> {
    
    pub fn new(base:NonNull<T>) -> Self {
        let base = unsafe{NonNull::new_unchecked(base.as_ptr() as *mut u8)};
        Self{
            base: base.as_ptr(),
            head: None,
            tail: None,
            len: 0,
            phantom: PhantomData,
        }
    }
    
    pub fn new_absolute() -> Self {
        Self{
            base: null_mut(),
            head: None,
            tail: None,
            len: 0,
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, mut memory:NonNull<T>, value:T) -> Result<(), ()> {
        let memory_mut = unsafe{memory.as_mut()};
        *memory_mut = value;
        self.insert_mem(memory)
    }
    
    pub fn insert_mem(&mut self, memory:NonNull<T>) -> Result<(), ()> {
        
        let anchor = self.anchor_from_node(memory.as_ptr()) as *mut u8;//unsafe{(memory.as_ptr() as *mut u8).sub(OFFSET)};
        let mut anchor_nn = unsafe{NonNull::new_unchecked(anchor as *mut LinkedListAnchor)};
        let anchor_mut = unsafe{anchor_nn.as_mut()};
        
        if anchor_mut.next.is_some() || anchor_mut.prev.is_some() {
            return Err(());
        }
        
        let relative_addr = self.rel_from_abs(anchor as *mut LinkedListAnchor);
        self.len += 1;
        
        match self.tail {
            Some(offset) => {
                let mut pivot = unsafe{NonNull::new_unchecked(self.base.offset(offset) as *mut LinkedListAnchor)};
                let pivot_mut = unsafe{pivot.as_mut()};
                let ptr_diff = unsafe{(anchor).offset_from(pivot.as_ptr() as *mut u8)};
                pivot_mut.next = Some(ptr_diff);
                anchor_mut.prev = Some(-ptr_diff);
                self.tail = Some(relative_addr);
                Ok(())
            }
            None => {
                self.head = Some(relative_addr);
                self.tail = Some(relative_addr);
                Ok(())
            }
        }
    }
    
    pub fn update_base(&mut self, new_base:*mut u8) {
        self.base = new_base;
    }
    
    pub fn get(&mut self, index:usize) -> Result<NonNull<T>, ()> {
        
        if self.len < index {
            return Err(());
        }
        
        let mut pivot_offset = match self.head { 
            Some(offset) => {offset}
            None => {return Err(());}
        };
        
        let mut pivot_anchor_u8 = unsafe{self.base.offset(pivot_offset)};
        let mut pivot_anchor = pivot_anchor_u8 as *mut LinkedListAnchor;
        let mut pivot_anchor_mut = unsafe{pivot_anchor.as_mut().expect("should be pointing to a node")};
        
        for _ in 0..index {
            pivot_offset = pivot_anchor_mut.next.expect("should be at least index size long to arrive to this point");
            pivot_anchor_u8 = unsafe{pivot_anchor_u8.offset(pivot_offset)};
            pivot_anchor = pivot_anchor_u8 as *mut LinkedListAnchor;
            pivot_anchor_mut = unsafe{pivot_anchor.as_mut().expect("should be pointing to a node")};
        }
        Ok(NonNull::new(self.node_from_anchor(pivot_anchor)).expect("should be pointing to a node"))
    }
    

    pub fn unlink(&mut self, node:NonNull<T>) {
        
        let anchor_u8 = self.anchor_from_node(node.as_ptr()) as *mut u8;//unsafe{(memory.as_ptr() as *mut u8).sub(OFFSET)};
        let anchor_mut = unsafe{NonNull::new_unchecked(anchor_u8 as *mut LinkedListAnchor).as_mut()};
        match (anchor_mut.prev, anchor_mut.next) {
            (Some(prev_offset), Some(next_offset)) => {
                let prev_mut = unsafe{NonNull::new_unchecked(anchor_u8.offset(prev_offset) as *mut LinkedListAnchor).as_mut()};
                let next_mut = unsafe{NonNull::new_unchecked(anchor_u8.offset(next_offset) as *mut LinkedListAnchor).as_mut()};
                let prev_next_offset = prev_mut.next.expect("should not be the last element");
                let next_prev_offset = next_mut.prev.expect("should not be the first element");
                prev_mut.next = Some(prev_next_offset+next_offset);
                next_mut.prev = Some(next_prev_offset+prev_offset);
                *anchor_mut = LinkedListAnchor::default();
            }
            (Some(prev_offset), None) => {
                let prev_u8 = unsafe{anchor_u8.offset(prev_offset)};
                let prev_mut = unsafe{NonNull::new_unchecked(prev_u8 as *mut LinkedListAnchor).as_mut()};
                prev_mut.next = None;
                self.tail = unsafe{Some(prev_u8.offset_from(self.base))};
                *anchor_mut = LinkedListAnchor::default();
                
            }
            (None, Some(next_offset)) => {
                let next_u8 = unsafe{anchor_u8.offset(next_offset)};
                let next_mut = unsafe{NonNull::new_unchecked(next_u8 as *mut LinkedListAnchor).as_mut()};
                next_mut.prev = None;
                self.head = unsafe{Some(next_u8.offset_from(self.base))};
                *anchor_mut = LinkedListAnchor::default();
            }
            (None, None) => {
                if self.head.is_none() && self.tail.is_none() {
                    return;
                }
                self.head = None;
                self.tail = None;
            }
        }
        self.len -= 1;
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    fn rel_from_abs(&self, address:*mut LinkedListAnchor) -> isize {
        unsafe{(address as *const u8).offset_from(self.base)}
        
    }
    
    fn anchor_from_node(&self, node:*const T) -> *mut LinkedListAnchor {
        (unsafe{(node as *mut u8).sub(OFFSET)}) as *mut LinkedListAnchor
    }
    
    
    fn node_from_anchor(&self, node:*const LinkedListAnchor ) -> *mut T {
        (unsafe{(node as *mut u8).add(OFFSET)}) as *mut T
    }
    
}


