
use super::*;

extern crate std;
use std::{println, dbg};

#[derive(Debug)]
struct TestStruct {
    value:i32,
    anchor:LinkedListAnchor,
}

#[test]
fn test_extern() {
    const OFFSET:usize = memoffset::offset_of!(TestStruct, anchor);
    let mut allocator = dyn_array::Array::<TestStruct>::new().unwrap();
    let mut address = allocator.base().as_ptr() as *mut u8;
    let address_ptr = std::ptr::addr_of_mut!(address);
    
    
    let mut holder:LinkedList<OFFSET, TestStruct> = LinkedList::new_extern(address_ptr);
    println!("{address:?}");
    for index in 0..128 {
        let (new_base, memory) = allocator.allocate().unwrap();
        match new_base {
            Some(new_base) => {
                address = new_base as *mut u8;
                dbg!(address);
                let _ = holder.insert(memory, TestStruct{value:index, anchor:LinkedListAnchor::default()});
            }
            None => {
                let _ = holder.insert(memory, TestStruct{value:index, anchor:LinkedListAnchor::default()});
            }
        }
    }
    
    let node = holder.get(5).unwrap();
    let node_ref = unsafe{node.as_ref()};
    dbg!(node_ref.value);
    
    
    let node = holder.get(0).unwrap();
    println!("{:?}", unsafe{node.as_ref()});
    holder.unlink(node);
    let node_ref = unsafe{node.as_ref()};
    println!("{} {:?}", node_ref.value, node_ref);
    
    let node = holder.get(1).unwrap();
    println!("{:?}", unsafe{node.as_ref()});
    holder.unlink(node);
    println!("{:?}", unsafe{node.as_ref()});
    
    for _ in 0..holder.len() {
        let node = holder.get(0).unwrap();
        holder.unlink(node);
    }
    
    //todo!("\n{holder:#?}");
}



#[test]
fn test_extern_iter() {
    const OFFSET:usize = memoffset::offset_of!(TestStruct, anchor);
    let mut allocator = dyn_array::Array::<TestStruct>::new().unwrap();
    let mut address = allocator.base().as_ptr() as *mut u8;
    let address_ptr = std::ptr::addr_of_mut!(address);
    
    
    let mut holder:LinkedList<OFFSET, TestStruct> = LinkedList::new_extern(address_ptr);
    
    println!("{address:?}");
    
    for index in 0..10 {
        let (new_base, memory) = allocator.allocate().unwrap();
        match new_base {
            Some(new_base) => {
                address = new_base as *mut u8;
                dbg!(address);
                let _ = holder.insert(memory, TestStruct{value:index, anchor:LinkedListAnchor::default()});
            }
            None => {
                let _ = holder.insert(memory, TestStruct{value:index, anchor:LinkedListAnchor::default()});
            }
        }
    }
    
    for (elem, index) in holder.iter_mut().zip(0..10) {
        let elem_ref = unsafe{elem.as_ref().unwrap()};
        assert!(elem_ref.value == index);
        dbg!(elem_ref);
    }
    
    //todo!("\n{holder:#?}");
}

/*
#[test]
fn test_array(){
    let mut holder:Array<i32> = Array::with_capacity(100).unwrap();
    println!("{holder:#?}");
    let _ = holder.push(9999);
    for elem in 0..1024{
        let _ = holder.push(elem);
    }
    println!("{holder:#?}");
    for _ in 0..1024 {
        let _ = holder.pop();
    }
    println!("{holder:#?}");
    for elem in 0..10{
        let _ = holder.push(elem);
    }
    println!("{holder:#?}");
}
*/
