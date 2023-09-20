use core::iter::{
    //FromIterator,
    Iterator,
    DoubleEndedIterator,
};

use super::{
    LLAnchor,
    LinkedList,
};

use core::{
    ptr::NonNull,
};

pub struct LLIter<'a, T> {
    pub(crate) reference: &'a LinkedList<T>,
    pub(crate) internal:LLIterInternals,
}

pub struct LLIterMut<'a, T> {
    pub(crate) reference: &'a mut LinkedList<T>,
    pub(crate) internal:LLIterInternals,
}

#[derive(Debug)]
pub(crate) struct LLIterInternals {
    pub(crate) current:Option<NonNull<LLAnchor>>,
    pub(crate) current_back:Option<NonNull<LLAnchor>>,
    pub(crate) finished: bool,
}

impl<'a, T> Iterator for LLIter<'a, T> {
    type Item = NonNull<T>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.reference.anchor_to_node(self.internal.next()?))
    }
}

impl<'a, T> DoubleEndedIterator for LLIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.reference.anchor_to_node(self.internal.next_back()?))
    }
}

impl<'a, T> Iterator for LLIterMut<'a, T> {
    type Item = NonNull<T>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.reference.anchor_to_node(self.internal.next()?))
    }
}

impl<'a, T> DoubleEndedIterator for LLIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.reference.anchor_to_node(self.internal.next_back()?))
    }
}

impl Iterator for LLIterInternals {
    type Item = NonNull<LLAnchor>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(pivot) => {
                if self.finished {
                    return None;
                }
                if self.current == self.current_back {
                    self.finished = true;
                } else {
                    let pivot_ref = unsafe{pivot.as_ref()};
                    self.current = pivot_ref.next;
                }
                return Some(pivot);
            }
            None => {
                return None;
            }
        }
    }
}

impl DoubleEndedIterator for LLIterInternals {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_back.clone() {
            Some(pivot) => {
                if self.finished {
                    return None;
                }
                if self.current == self.current_back {
                    self.finished = true;
                } else {
                    let pivot_ref = unsafe{pivot.as_ref()};
                    self.current_back = pivot_ref.prev;
                }
                return Some(pivot);
            }
            None => {
                return None;
            }
        }
    }
}
