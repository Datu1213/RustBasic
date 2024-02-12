#![allow(unused)]
use num::complex::Complex;
fn basic_type() {
    //////////////////////////////////////////////////////////////
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

    //////////////////////////////////////////////////////////////
    // Use "mut" to make it mutable
    let mut e: u8 = 0;
    e = 1;
    
    // Unused variables will cause warning, use prefix "_" to let Rust ignore this kind of warning.
    let _f: u8 = 1; 
    
    //////////////////////////////////////////////////////////////
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

    // Use "as" to transmute variable's type
    let x = 0x2F as i32;
    print!("{x}\n");

    // "let guess = "42".parse::<i32>().expect("Not a number!");" Rust will confuse, it can't deduce what type this variable is.
    // Make some indication manually.
    let guess = "42".parse::<i32>().expect("Not a number!");
    // Or "let guess: i32 = "42".parse().expect("Not a number!");"
    print!("{guess}\n");

    // Seperate a longer number with "_".
    let one_million: i64 = 1_000_000;
    // It's same to "let one_million: i64 = 1000000;".
    println!("{}", one_million.pow(2));
}

fn greet_world() {
    let southern_germany: &str  = "Grüß Gott!";
    let chinese: &str           = "世界，你好";
    let english: &str           = "World, hello";
    let regions: [&str; 3] = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region);
    }
    println!("{}", &chinese);
}

fn intrinsic_type() {
    // Intrinsic type
    let int_size: isize = 1; // Depends on the CPU
    let uint_size: usize = 1; // Depends on the CPU
    // My computer is 64bits == 8Byte, so both output should be 8
    println!("isize: {}, usize: {}", std::mem::size_of_val(&int_size), std::mem::size_of_val(&uint_size));
}

fn float_trap() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits()); // Raw transmutation to u32 or u64, and print it as hexadecimal type.
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
}

fn int_overflowing() {
    // assert!(0.1 + 0.2 == 0.3); // ERROR!

    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);  

    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19
    
    let c = a.checked_add(20); 
    println!("{:?}", c);
}

fn not_a_number() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("This is not a number!")
    }
}

fn complex_number() {
    let a = Complex{ re: 1, im: 1};
    let b = Complex::new(2, 3);
    let result_sum = a + b;
    let result_mul = a * b;
    println!("({:3}   + {:3}i)   + ({:3}   + {:3}i) = ({:3}   + {:3}i)", a.re, a.im, b.re, b.im, result_sum.re, result_sum.im);
    println!("({:3}   + {:3}i)   * ({:3}   + {:3}i) = ({:3}   + {:3}i)", a.re, a.im, b.re, b.im, result_mul.re, result_mul.im);
}

fn character() {
    // Rust use Unicode: 4Bytes, not traditional char: 1Byte.
    let c = 'z';
    let z = 'ℤ'; // Styled "Z".
    let g = '国'; // Chinese character.
    let heart_eyed_cat = '😻'; // Emoji character.
    //////////////// Use `'`, not `"`.
    println!("Size of characters: {}, {}, {}, {}", std::mem::size_of_val(&c), std::mem::size_of_val(&z), std::mem::size_of_val(&g), std::mem::size_of_val(&heart_eyed_cat));
}

fn adddd(x: i32, y: i32) -> i64 {
    return x as i64 * y as i64;
}

fn unit_type() {
    // Rust function always have a return type;
    // It is not "void", it's unit type ----> "()". 

    // Code "println!("{:#?}", unit_type());" to see what will be displayed.
    return;
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // Statement : Do something without return value.
    let y = y + 5; // Statement : Do something without return value.
    x + y               // Expression: Do something with    return value.
    // No semicolon.
}
fn main() {
    // greet_world();
    
    // basic_type();

    // intrinsic_type();

    // int_overflowing();

    // float_trap();

    // not_a_number();

    // let a = 34 as u32;
    // println!("Decimal:     {:6 }", a);
    // println!("Hexadecimal: {:6x}", a);
    // println!("Octal:       {:6o}", a);
    // println!("Binary:      {:6b}", a);

    // complex_number();

    // character();

    // println!("{}", adddd(1, 2));

    // println!("{:#?}", unit_type()); 
}