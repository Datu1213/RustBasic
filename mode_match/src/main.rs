#![allow(unused)]

use std::{array, path::Display};
#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}
///
// match target {
//     模式1 => 表达式1,
//     模式2 => {
//         语句1;
//         语句2;
//         表达式2
//     },
//     _ => 表达式3
// }
///
fn match_test() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}

fn match_and_assign() {
    let ip1 = Direction::West;
    let ip_str = match ip1 {
        Direction::West => "West",
        _ => "::1",
    };

    println!("{}", ip_str); // West
}

fn macros_matches() {
    let dir_array = [1, 2, 3];
    let mut filted = dir_array.iter().filter(|x| (**x) > 1);
    println!("{}", filted.next().unwrap());
    println!("{}", filted.next().unwrap());
    // assert!(matches!([Direction::South], dir_array.iter().filter(|x| matches!(x, Direction::South))))
}

fn only_one_match() {
    let v = Some(3);
    if let Some(3) = v { // Use "if let `option` = `value`", if only one option/mode will be matched.
        println!("three");
    }
}

fn shadowing_in_match() {
    let age = Some(30);
    println!("Before: {:?}",age);
    match age {
        // Some(age) =>  println!("Matched: {}", age), // Shadow
        Some(x) =>  println!("Matched: {}", x), // Better way.
        _ => ()
    }
   println!("After: {:?}",age);
}

fn deconstruct_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);
}

fn while_let_loop() {
    // Mutable.
    let mut stack = Vec::new();

    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // Mutable iterator.
    let mut iter_stack = stack.iter();
    
    while let Some(top) = iter_stack.next() {
        println!("{}", top);
    }
}

fn enumerate() {
    let v = vec!['a', 'b', 'c'];

    // Tuple(index, value).
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
}
}

fn pattern_match() {
    // Every `=` is a `match`!!!!
    //
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }
    //
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     _ => EXPRESSION,
    // }
    //
    // Variable's name is also a pattern.
    // let PATTERN = EXPRESSION;
    //
    // if let PATTERN = SOME_VALUE {
    //
    // }
    //
    // fn foo(x: i32) {
    //     // Some code.
    // }
    // 
    // let Some(x) = some_option_value; // Error.
    // `let` can't cover all options.
    // But `if let` can do it.
}

fn or_match() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
fn main() {
    // macros_matches();

    // only_one_match();

    // shadowing_in_match();

    // deconstruct_option();

    // while_let_loop();
}

