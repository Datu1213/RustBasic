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

    // "let" means a immutable variable, it can not change after assigned to a value.
    // "d = 2;"  will cause an error: "cannot assign twice to immutable variable".

    // Use "mut" to make it mutable
    let mut e: u8 = 0;
    e = 1;
    

    
}