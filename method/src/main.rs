#![allow(unused)]

#[derive(Debug)]
pub struct Rectangle { // Struct
    width: u32, 
    height: u32,
}

// Use `impl`
impl Rectangle { // Methods of Struct
    pub fn new(width: u32, height: u32) -> Self { // `Self` means the Strut we use, it's Rectangle now.
        return Rectangle {
            width,
            height
        };
    }

    pub fn area(&self) -> u32 { // `&self` is abbreviation of `self: &Self`
        
        return self.width * self.height;
    }

    pub fn width(&self) -> u32{ // Members are private for files outside.
        return self.width; // Methods' names can be as same as members'.
    } // It always used to make getter function.

    pub fn height(&self) -> u32{
        return self.height;
    }
}


fn main() {
    let rec = Rectangle::new(1, 2);
    
    println!("{:3}", rec.width);
}
