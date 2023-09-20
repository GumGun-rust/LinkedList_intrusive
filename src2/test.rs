use super::*;

extern crate std;
use std::println;
use std::boxed::Box;


#[derive(Debug, Default)]
struct Holder {
    value:u64,
    anchor:LLAnchor,
}


#[test]
fn new() {
    let mut list = unsafe{LinkedList::<Holder>::new(offset_of!(Holder, anchor))};
    
    println!("{}", offset_of!(Holder, anchor));
    println!("{}", offset_of!(Holder, value));
    println!("{:#?}", list);
    
    let arr_holder:[Holder; 5] = Default::default();
    let mut arr = NonNull::new(Box::into_raw(Box::new(arr_holder))).expect("mem error");
    
    let current = unsafe{arr.as_mut()};
    println!("{:#?}", current);
    current[1].value = 1;
    current[2].value = 2;
    //let node = NonNull::new(Box::into_raw(Box::new(Holder{value:1,..Holder::default()}))).expect("mem error");
    let arr_ptr = current.as_mut_ptr();
    let node0 = unsafe{NonNull::new_unchecked(arr_ptr)};
    let node1 = unsafe{NonNull::new_unchecked(arr_ptr.add(1))};
    list.append(node1).unwrap();
    
    let node2 = unsafe{NonNull::new_unchecked(arr_ptr.add(2))};
    list.append(node2).unwrap();
    
    let get = list.get(1).unwrap();
    let get = unsafe{get.as_ref()};
    println!("value from get{:#?}", get);
    
    assert_eq!(list.has(node0), false);
    assert_eq!(list.has(node1), true);
    
    println!("{:#?}", current);
    //panic!();
}

#[test]
fn iter_list() {
    let mut list = unsafe{LinkedList::<Holder>::new(offset_of!(Holder, anchor))};
    
    let arr_holder:[Holder; 5] = Default::default();
    let mut arr = NonNull::new(Box::into_raw(Box::new(arr_holder))).expect("mem error");
    
    let current = unsafe{arr.as_mut()};
    println!("{:#?}", current);
    current[0].value = 0;
    current[1].value = 1;
    current[2].value = 2;
    current[3].value = 3;
    let arr_ptr = current.as_mut_ptr();
    
    let node0 = unsafe{NonNull::new_unchecked(arr_ptr)};
    list.append(node0).unwrap();
    
    let node1 = unsafe{NonNull::new_unchecked(arr_ptr.add(1))};
    list.append(node1).unwrap();
    
    let node2 = unsafe{NonNull::new_unchecked(arr_ptr.add(2))};
    list.append(node2).unwrap();
    
    let node3 = unsafe{NonNull::new_unchecked(arr_ptr.add(3))};
    list.append(node3).unwrap();
    

    for elem in list.iter() {
        println!("{:?}", unsafe{elem.as_ref()});
    }
    
    let mut iter = list.iter();
    println!("{:?}", unsafe{iter.next().unwrap().as_ref()});
    println!("{:?}", unsafe{iter.next().unwrap().as_ref()});
    
    let mut iter = iter.rev();
    println!("{:?}", unsafe{iter.next().unwrap().as_ref()});
    println!("{:?}", unsafe{iter.next().unwrap().as_ref()});
    
    /*
    for elem in .rev() {
        println!("{:?}", unsafe{elem.as_ref()});
    }
    */
    
    //println!("{:#?}", current);
    panic!();
}

#[test]
fn iter() {
    let mut iter = 0..=10;
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    let mut iter = iter.rev();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    let mut iter = iter.rev();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    //panic!();
}

