#![no_std]
#[cfg(test)]
mod test;

mod iter;
pub use iter::{
    LLIter,
    LLIterMut,
};

use core::{
    marker::PhantomData,
    ptr::NonNull,
};

pub use memoffset;
pub use memoffset::offset_of;

#[derive(Debug, Default)]
pub struct LLAnchor {
    prev:Option<NonNull<LLAnchor>>,
    next:Option<NonNull<LLAnchor>>,
}

#[derive(Debug, Default)]
pub struct LinkedList<T> {
    head:Option<NonNull<LLAnchor>>,
    tail:Option<NonNull<LLAnchor>>,
    offset:usize,
    size:usize,
    phantom: PhantomData<T>,
}


impl<T> LinkedList<T> {
    
    pub unsafe fn new(offset:usize) -> Self {
        Self{
            head:None,
            tail:None,
            offset:offset,
            size:0,
            phantom:PhantomData,
        }
    }
    
    pub fn insert(&mut self, node:NonNull<T>) -> Result<(), ()> {
        let mut anchor_nn = self.node_to_anchor(node);
        let anchor_mut = unsafe{anchor_nn.as_mut()};
        if anchor_mut.prev.is_some() || anchor_mut.next.is_some() {
            return Err(())
        }
        match self.head {
            None => {
                self.head = Some(anchor_nn);
                self.tail = Some(anchor_nn);
                self.size += 1;
            }
            Some(mut pivot) => {
                let mut pivot_mut = unsafe{pivot.as_mut()};
                while let Some(mut new_pivot) = pivot_mut.next {
                    pivot_mut = unsafe{new_pivot.as_mut()};
                    pivot = new_pivot;
                }
                pivot_mut.next = Some(anchor_nn);
                anchor_mut.prev = Some(pivot);
                self.tail = Some(anchor_nn);
                self.size += 1;
            }
        }
        Ok(())
    }
    

    pub fn get(&mut self, mut index:usize) -> Result<NonNull<T>, ()> {
        match self.head {
            None => {
                Err(())
            }
            Some(mut pivot) => {
                while index > 0 {
                    index -= 1;
                    let pivot_ref = unsafe{pivot.as_ref()};
                    match pivot_ref.next {
                        Some(new_pivot) => {
                            pivot = new_pivot;
                        }
                        None => {
                            return Err(());
                        }
                    }
                }
                Ok(self.anchor_to_node(pivot))
            }
        }
    }

    
    pub fn delete(&mut self, node:NonNull<T>) -> Result<(), ()> {
        let mut anchor_nn = self.node_to_anchor(node);
        let anchor_mut = unsafe{anchor_nn.as_mut()};
        match anchor_mut.prev {
            Some(mut prev) => {
                let prev_mut = unsafe{prev.as_mut()};
                prev_mut.next = anchor_mut.next;
            }
            None => {
                self.head = anchor_mut.next;
            }
        }
        match anchor_mut.next {
            Some(mut next) => {
                let next_mut = unsafe{next.as_mut()};
                next_mut.next = anchor_mut.prev;
            }
            None => {
                self.tail = anchor_mut.prev;
            }
        }
        anchor_mut.prev = None;
        anchor_mut.next = None;
        Ok(())
    }
    

    pub fn has(&self, node:NonNull<T>) -> bool {
        let anchor_nn = self.node_to_anchor(node);
        let anchor_ref = unsafe{anchor_nn.as_ref()};
        if anchor_ref.prev.is_some() || anchor_ref.next.is_some() {
            true
        } else {
            false
        }
    }
    

    
    pub fn len(&self) -> usize {
        self.size
    }
    
    
    pub fn iter(&self) -> iter::LLIter<T> {
        LLIter{
            reference: self,
            internal: iter::LLIterInternals{
                current: self.head,
                current_back: self.tail,
                finished: false,
            }
        }
    }
    
    pub fn iter_mut(&mut self) -> iter::LLIterMut<T> {
        let holder = iter::LLIterInternals{
            current: self.head,
            current_back: self.tail,
            finished: false,
        };
        LLIterMut{
            reference: self,
            internal: holder,
        }
    }
    
    
    #[inline(always)]
    pub fn anchor_to_node(&self, anchor:NonNull<LLAnchor>) -> NonNull<T> {
        let holder = unsafe{(anchor.as_ptr() as *mut i8).sub(self.offset)};
        unsafe{NonNull::new_unchecked(holder as *mut T)}
    }

    
    #[inline(always)]
    pub fn node_to_anchor(&self, node:NonNull<T>) -> NonNull<LLAnchor> {
        let anchor = unsafe{(node.as_ptr() as *mut i8).add(self.offset)};
        unsafe{NonNull::new_unchecked(anchor as *mut LLAnchor)}
    }
    
    
}


