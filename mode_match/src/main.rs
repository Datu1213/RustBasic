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

fn main() {
    macros_matches();
}

