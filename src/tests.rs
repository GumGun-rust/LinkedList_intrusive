use super::*;

extern crate std;
use std::println;

#[derive(Debug)]
struct TestStruct {
    value:i32,
    anchor:LinkedListAnchor,
}

use dyn_array::Array;

#[test]
fn test() {
    const OFFSET:usize = memoffset::offset_of!(TestStruct, anchor);
    let mut allocator = dyn_array::Array::<TestStruct>::new().unwrap();
    let base = allocator.base();
    let mut holder:LinkedList<OFFSET, TestStruct> = LinkedList::new(base);
    
    //for index in 0..128 {
    for index in 0..4 {
        let (new_base, memory) = allocator.allocate().unwrap();
        match new_base {
            Some(new_base) => {
                holder.update_base(new_base);
                let _ = holder.insert(memory, TestStruct{value:index, anchor:LinkedListAnchor::default()});
            }
            None => {
                let _ = holder.insert(memory, TestStruct{value:index, anchor:LinkedListAnchor::default()});
            }
        }
    }
    
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
    
    
    /*
    let (new_base, memory) = allocator.allocate().unwrap();
    let _ = holder.insert(memory, TestStruct{value:2, anchor:LinkedListAnchor::default()});
    let (new_base, memory) = allocator.allocate().unwrap();
    let _ = holder.insert(memory, TestStruct{value:3, anchor:LinkedListAnchor::default()});
    let (new_base, memory) = allocator.allocate().unwrap();
    let _ = holder.insert(memory, TestStruct{value:4, anchor:LinkedListAnchor::default()});
    */
    
    todo!("\n{holder:#?}\n{allocator:#?}");
}

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

