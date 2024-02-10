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
    let (a, mut b): (bool,bool) = (true, false); // Reclaim variables "a" and "b"
    // a = true;    immutable 
    // b = false;   mutable
    println!("a = {:?}, b = {:?}", a, b);
    // Output: a = true, b = false
    b = true;
    
    assert_eq!(a, b);

    const PI: f64 = 3.1415926;
    // Reclaiming "const PI: f64 = 3.1415926;" will cause an error,
    // and it's the difference between "let" variable and "const" constant.
}