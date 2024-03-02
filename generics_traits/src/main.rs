#![allow(unused)]

use std::fs::File;
use std::ops::Deref;
use std::process::Output;
use std::{fmt::Debug, ops::Add};

fn add<T: Add<Output = T>>(a:T, b:T) -> T {
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

//////////////// Use Generic in methods.
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
    // No `{}`, use `;`.
    // fn speak(&self); // Without default implement.

    fn speak(&self) {
        println!("Heng, heng, AAAAAAAAAAAAAA");
    } // No `;`, use `{}`.
}

pub struct Cat {

}

impl Speak for Cat {
    fn speak(&self) {
        println!("Mew.");
    }
}

pub struct Dog {
    
}

impl Speak for Dog {
    fn speak(&self) {
        println!("Wolf.");
    }
}

pub struct Homo {
    
}

impl Speak for Homo {
// Use default implement.
}

pub trait Crying {
    fn cry(&self);
}

// Make implement of Crying for every type who has made implement of Speak.
// If you can speak, you can cry.
impl<T: Speak> Crying for T {
    fn cry(&self) {
        println!("WAAAAAAAAAAAAAAAA!!!!!");
    }
}
// Or
// impl<T> Crying for T 
// where T: Speak
// {
//     fn cry() {
//         println!("WAAAAAAAAAAAAAAAA!!!!!");
//     }
// }

// Orphan rule. //
// If you want to make a implement of a trait for a strcut,
// at least one of them----the "Strcut" or the "Trait" is claimed in current scope.

// Use trait in function arguments.
// Term candy:
fn foo(i: &impl Speak, j: &impl Speak) {
    ///////////////////////////////////////////////
    // Motify that term `&` ignore a backet `()` //
    // Integral statement: &(impl Speak)         //
    ///////////////////////////////////////////////
}
//////////////////////////

// Normal way.
fn bar<T: Speak, U: Speak>(i: &T, j: &U) {
    
}

// If "i" is as same as "j", use normal way
fn baz<T: Speak>(i: &T, j: &T) {
    
}

// Multipul trait bounds.
fn foo_mtb(i: &(impl Speak + Crying)) {

}

fn trait_bound_where<T, U>(i: &T, j: &U) 
    where T: Speak,
          U: Crying
{
    // Snip.
}

// Trait in return value.
fn returns_something_can_speak() -> impl Speak {
    return Cat {};
}

// Use `#[derive(Trait)]` to make a default implement.
// Only availible for those trait which is intrincic in Rust standard.


////////////////////////////////////
// If you want to use a method of a trait, you shoud introduce this trait.
///////////////////////////////////

// Example: Add
// use std::ops::Add
#[derive(Debug)]
struct Point<T: Add<Output = T>> {
    x: T,
    y: T
}
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
     fn add(self, rhs: Self) -> Self::Output {
         return Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
         }
     }
}

//Error now.
// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum FileState {
    Open,
    Closed
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Self::Open => write!(f, "Open"),
            Self::Closed => write!(f, "Cloesd")
        }
    }
}

// Dynamic Traits
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String{
        format!("Size: {} X {}, Label: {}", self.width, self.height, self.label)
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
    value: String
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!("Size: {} X {}, Label: {}", self.width, self.height, self.value)
    }
}
pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}


// Associated type 
trait AS {
    type ASType;
    type ASType2: Clone + Display + Debug;
    // Associated type make it more clear to when you make an implement of trait.
    // You don't have to rewrite `impl<Type_1, Type_2...>`.
    // But Associated types don't have defualt names.
}

///////////////////////////////////////////
// Defualt Generics type.                //
// See the example of traitstd::op::Add. //
///////////////////////////////////////////


// impl Add with different member being added.
struct Millimeters(u32);
struct Meters(u32);
struct Centimeters(u32);

impl Add<Millimeters> for Meters{
    type Output = Centimeters;
    fn add(self, rhs: Millimeters) -> Self::Output {
        Centimeters(self.0 + rhs.0 * 1000)
    }
}
// Meters + Millimeters => Centimeters
// That's why `Add<Millimeters, Output = Centimeters>` is so beautiful.



// Methods with a same name.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Suiiiiiiiiiiiiiiiiiiiiii.");
    }
}
// Now Human has three fly(&self)
fn sanme_name_methods() {
    let humen = Human{};
    humen.fly(); // Defualt method.  "Suiiiiiiiiiiiiiiiiiiiiii"
    Pilot::fly(&humen); // "Up!"
    Wizard::fly(&humen); // "This is your captain speaking."
}

fn same_name_associated_functions() {
    trait Animal {
        fn baby_name() -> String;
    }
    
    struct Dog;
    
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // Use baby_name() implated in Animal.
    println!("{}", Dog::baby_name()); // Spot
    // Complet constrain statement.
    println!("{}", <Dog as Animal>::baby_name()); // puppy
}

// Avoid Orphan rule with new type
fn anti_orphan() {
    struct Wrapper(Vec<String>);
    impl Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "[{}]", self.join(", "))
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // let arr = [1, 2, 3];
    // const_generic(arr);
    // let arr = [1, 2, 3, 4];
    // const_generic(arr);

    let cat = Cat{};
    let dog = Dog{};
    let homo = Homo{};
    cat.speak();
    dog.speak();
    homo.speak();
    cat.cry();
    dog.cry();
    homo.cry();

    let open = FileState::Open;
    println!("{}", open);
    println!("{:?}", open);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
                value: String::from("Yes")
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    ////////////
    let p_1 = Point {
        x: 0.1,
        y: 0.2
    };
    let p_2 = Point {
        x: 0.3,
        y: 0.2
    };

    println!("{:?}", p_1 + p_2);

    same_name_associated_functions();
}
