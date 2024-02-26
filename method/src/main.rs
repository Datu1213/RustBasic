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


// impl for enum 
enum Message {
    Quit(i32),
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn process(&self) {
        match self {
            Self::Quit(-1) => {
                println!("Bye!!");
            }
            Self::ChangeColor(r, g, b) => {
                println!("R: {}, G: {}, B: {}", r, g, b);
            }
            Self::Move { x, y } => {
                println!("Move to:({}, {})", x, y);
            }
            Self::Write(str) => {
                println!("{}", str);
            }
            _ => {}
        }
    }
}
fn main() {
    let rec = Rectangle::new(1, 2);
    
    println!("{:3}", rec.width);

    let m = Message::Write(String::from("hello"));
    m.process();
}
