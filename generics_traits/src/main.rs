#![allow(unused)]

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
   return a + b
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { // std::cmp::PartialOrd is a Trait
    let mut largest = &list[0];

    for item in list.iter() {
        if *item > *largest {     
            largest = item;
        }
    }

    return largest; // Most safe way.
}

pub struct Rectangle<T> { // Struct
    width: T, 
    height: T,
}

// Use `impl<T>`.
impl<T> Rectangle<T>{
    fn width(&self) -> &T {
        return &self.width;
    }
}

// Only `impl`.
// For specific generic type.
impl Rectangle<f32> {
    fn diagnoal_length(&self) -> f32 {
        (self.width.powi(2) + self.width.powi(2)).sqrt()
    }
}

// Const generic
// Const generic is value-wise, while common generic is type-wise.
fn const_generic<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    for item in arr.iter() {
        print!("{:?}", item);
    }
    print!("\n");
}

// Trait / Interface
pub trait Speak {
    fn speak(&self);
}

pub struct Cat {

}

impl Speak for Cat {
    fn speak(&self) {
        println!("Mew.")
    }
}

pub struct Dog {
    
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Wolf.")
    }
}

// Orphan rule.
// If you want to make a implement of a trait for a strcut,
// at least one of them----the "Strcut" or the "Trait" is claimed in current scope.

fn main() {
    // println!("Hello, world!");
    // let arr = [1, 2, 3];
    // const_generic(arr);
    // let arr = [1, 2, 3, 4];
    // const_generic(arr);

    let cat = Cat{};
    let dog = Dog{};
    cat.speak();
    dog.speak();
}
