#![allow(unused)]
fn for_loop() {
    for i in 1..=5 {
        println!("{}", i);
    }
    
}

fn for_move() {
    // for i in collection : Ownership will move.
    let strs: [String; 8] = std::array::from_fn(|_i| format!("It is {}", _i));
    for i in strs {
        println!("{}", i);
    }
    // let str: &String = &strs[0]; borrow of moved value
}

fn for_borrow() {
    // for i in &collection : Immutable reference.
    let strs: [String; 8] = std::array::from_fn(|_i| format!("It is {}", _i));
    for i in &strs {
        println!("{}", i);
    }
    let str: &String = &strs[0]; 
}

fn for_mut_borrow() {
    // for i in &mut collection : Mutable reference.
    let mut strs: [String; 8] = std::array::from_fn(|_i| format!("It is {}", _i));
    for i in &mut strs {
        println!("{}", i);
    }
    let str: &String = &strs[0]; 
}
fn main() {
    println!("Hello, world!");
}
