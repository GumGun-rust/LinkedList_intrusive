use super::{
    LinkedList,
    LinkedListAnchor,
};

use core::iter::{
    Iterator
};


pub struct IterMut<'a, const OFFSET:usize, T> {
    holder: &'a mut LinkedList<OFFSET, T>,
    current: Option<*mut LinkedListAnchor>,
}

impl<'a, const OFFSET:usize, T> IterMut<'a, OFFSET, T> {
    
    pub fn new(holder:&'a mut LinkedList<OFFSET, T>) -> Self {
        let current = holder.get_first();
        Self{
            holder: holder,
            current: current,
        }
    }
    
}

impl<'a, const OFFSET:usize, T> Iterator for IterMut<'a, OFFSET, T> {
    type Item = *mut T;
    
    fn next(&mut self) -> Option<Self::Item> {
        let pivot_anchor = self.current?;
        
        let holder = unsafe{pivot_anchor.byte_sub(OFFSET)};
        
        let pivot_anchor_mut = unsafe{pivot_anchor.as_mut().expect("should be pointing to a node")};
        
        self.current = pivot_anchor_mut.next.map(|offset|unsafe{pivot_anchor.byte_offset(offset)});
        
        Some(self.holder.node_from_anchor(holder))
    }
    
}

pub struct Iter<'a, const OFFSET:usize, T> {
    holder: &'a LinkedList<OFFSET, T>,
    current: Option<*mut LinkedListAnchor>,
}

impl<'a, const OFFSET:usize, T> Iter<'a, OFFSET, T> {
    
    pub fn new(holder:&'a LinkedList<OFFSET, T>) -> Self {
        let current = holder.get_first();
        Self{
            holder: holder,
            current: current,
        }
    }
    
}

impl<'a, const OFFSET:usize, T> Iterator for Iter<'a, OFFSET, T> {
    type Item = *const T;
    
    fn next(&mut self) -> Option<Self::Item> {
        let pivot_anchor = self.current?;
        
        let holder = unsafe{pivot_anchor.byte_sub(OFFSET)};
        
        let pivot_anchor_mut = unsafe{pivot_anchor.as_mut().expect("should be pointing to a node")};
        
        self.current = pivot_anchor_mut.next.map(|offset|unsafe{pivot_anchor.byte_offset(offset)});
        
        Some(self.holder.node_from_anchor(holder))
    }
    
}
