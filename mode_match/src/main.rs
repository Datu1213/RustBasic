#![allow(unused)]

use std::array;
#[derive(Debug)]
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

fn range_match() {
    let x = 1;

    match x {
        1..=5 => println!("One to five"),
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn deconstrucion_mach() {
    fn deconstrcut_struct() {
        struct Point {
            x: i32,
            y: i32,
        }
        
        let p = Point { x: 0, y: 7 };
    
        let Point { x: a, y: b } = p; // a and b are newly generated variables.
        assert_eq!(0, a);
        assert_eq!(7, b);

        // Or
        let Point { x, y } = p; // Use same names of x and y.
        assert_eq!(0, x);
        assert_eq!(7, y);

        // Match parts of the struct. 
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x), // Match y = 0
            Point { x: 0, y } => println!("On the y axis at {}", y), // Match x = 0
            Point { x, y } => println!("On neither axis: ({}, {})", x, y), // Others.
        }
    }
    fn deconstrcut_enum() {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        
        
            let msg = Message::ChangeColor(0, 160, 255);
        
            match msg {
                Message::Quit => {
                    println!("The Quit variant has no data to destructure.")
                }
                Message::Move { x, y } => {
                    println!(
                        "Move in the x direction {} and in the y direction {}",
                        x,
                        y
                    );
                }
                Message::Write(text) => println!("Text message: {}", text),
                Message::ChangeColor(r, g, b) => {
                    println!(
                        "Change the color to red {}, green {}, and blue {}",
                        r,
                        g,
                        b
                    )
                }
            }
        
    }
    fn deconstrcut_array() {
        // Fixed length
        let arr: [u16; 2] = [114, 514];
        let [x, y] = arr;

        assert_eq!(x, 114);
        assert_eq!(y, 514);

        // Unfixed length
        let arr: &[u16] = &[114, 514];

        if let [x, ..] = arr {
            assert_eq!(x, &114);
        }

        if let &[.., y] = arr {
            assert_eq!(y, 514);
        }

        let arr: &[u16] = &[];

        assert!(matches!(arr, [..]));
        assert!(!matches!(arr, [x, ..]));
    }
    deconstrcut_struct();
}

fn ignore() {
    // Use `_` to ignore value in patterns, arguments or arrays.
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    


    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);



    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }



    // `_` wil NOT bind any value.
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {    // if let Some(_s) = s {  Value will borrow if you use `_s`.
        println!("found a string");
    }

    println!("{:?}", s);

    // Use `..` to ignore the rest of values.
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    
    let origin = Point { x: 0, y: 0, z: 0 };
    
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // Ignore interval values.
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    // `..` has to be explicit.
    // let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     (.., second, ..) => {  // Error!!!!!!!
    //         println!("Some numbers: {}", second)
    //     },
    // }
}
fn main() {
    // macros_matches();

    // only_one_match();

    // shadowing_in_match();

    // deconstruct_option();

    // while_let_loop();

    // range_match();
    
    deconstrucion_mach();
}

