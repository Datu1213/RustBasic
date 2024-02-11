fn greet_world() {
    let southern_germany: &str = "Grüß Gott!";
    let chinese: &str = "世界，你好";
    let english: &str = "World, hello";
    let regions: [&str; 3] = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region);
    }
    println!("{}", &chinese);
}

fn main() {
    // println!("Hello, world!");

    // greet_world();

    // Basic value and type
    //              f --> float
    //              i --> int
    //              u --> unsigned int
    //  suffix number --> number of bits
    // Example: f64 --> float with 64bits
    let a = 3.1415926; // Without type indication.
    let b: f32 = 4.1415926; // With    type indication.
    let c = 3i32;      // With    type indication as a suffix of value.
    let d = 3_u32;     // With    type indication as a suffix of value, separated by "_".
    // "=" means "binding" but "assignment"

    // "let" means a immutable variable, it can be assigned again.
    // "d = 2;"  will cause an error: "cannot assign twice to immutable variable".

    // Use "mut" to make it mutable
    let mut e: u8 = 0;
    e = 1;
    
    // Unused variables will cause warning, use prefix "_" to let Rust ignore this kind of warning.
    let _f: u8 = 1; 
    

    // Variable deconstruction.
    // We use "Shadowing" here, reclaim and reassign variables "a" and "b".
    // It will create new memory spaces, new variable with same names, and maybe new variable type, 
    // and cover the previous one,
    // which means more performance costs, while "mut" will not.
    let (a, mut b): (bool,bool) = (true, false); 
    // b = false;   mutable
    println!("a = {:?}, b = {:?}", a, b);
    // Output: a = true, b = false
    b = true;
    
    assert_eq!(a, b); // Check if they are equal.

    const PI: f64 = 3.1415926;
    // Reclaiming "const PI: f64 = 3.1415926;" will cause an error,
    // and it's the difference between "let" variable and "const" constant.

    let x = 0x2F as i32;
    print!("{x}\n");

    // "let guess = "42".parse::<i32>().expect("Not a number!");" Rust will confuse, it can't deduce what type this variable is.
    // Make some indication manually.
    let guess = "42".parse::<i32>().expect("Not a number!");
    // Or "let guess: i32 = "42".parse().expect("Not a number!");"
    print!("{guess}\n");

    // Special type
    let int_size: isize = 1; // Depends on the CPU
    let uint_size: usize = 1; // Depends on the CPU
    // My computer is 64bits == 8Byte, so both output should be 8
    println!("isize: {}, usize: {}", std::mem::size_of_val(&int_size), std::mem::size_of_val(&uint_size));

    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);  

    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19

    assert!(0.1 + 0.2 == 0.3);// ERROR!
}