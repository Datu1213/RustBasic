#![allow(unused)]

fn string_slice() {
    let mut str1: String = String::from("Hello, world");
    let str_slice1: &str = &str1[1..2]; // slice from index 1 to 2.
    let str_slice2: &str = &str1[ ..2]; // slice from start   to 2.
    let str_slice3: &str = &str1[1.. ]; // slice from index 1 to end.
    let str_slice3: &str = &str1[ .. ]; // slice from start   to end.
    // All slices above are immutable reference!!!
}

fn int_slice() {
    let array_int: [i32; 5] = [1, 2, 3, 4, 5]; // Do not use range;
    let array_slice: &[i32] = &array_int[1..];
    assert_eq!(array_slice, &[2, 3, 4, 5]);
}

fn ref_scope_and_var_scope() {
    let mut str = String::from("String");

    let str_ref = &str[2..]; // immutable ref "str_ref" start 

    str.clear(); // mutable ref "&str" start

    // println!("{}", str_ref); // immutable ref "str_ref" end

    // Dirty Data Warning. 
}

fn chinese_clice() { // No string index in Rust.
    let str: String = String::from("中国话");
    let str_ref = &str[0..1];
    println!("{}", str_ref); //Error, chinese character occupies 3 Bytes.
    // byte index 1 is not a char boundary; it is inside '中' (bytes 0..3) of `中国话`
    // println!("{}", &str[1]); // Error.
}

fn push_string() {
    let mut str = String::from("a");

    // Push a fixed 4Bytes char.
    str.push('\u{5426}'); // Unicode char.
    // Char use `'` not `"`.

    // Push a String.
    str.push_str("bcd");

    println!("{}", str); // a否bcd
}

fn insert_string() {
    let mut s = String::from("Hello pineapple!");
    s.insert(5, ',');
    println!("insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("insert_str() -> {}", s);
}

fn replace_string() {
    let mut s = String::from("Hello pineapple!");
    s.replace("pineapple", "apple"); // Replace all matches of a pattern, change itself, use "&self".
    s = s.replacen("p", "P", 3); // Not intrinsic, it has a return value.

    s.replace_range(0..=4, "HELLO_suffix"); // They don't have to have a same length.
    // Change itself, use "&mut self". 
    // And no square backets "[]" for property "range".

    println!("{}", s); // HELLO_suffix PineaPPle!
}

fn pop_string() {
    let mut string_pop = String::from("pop 21");
    let p1 = string_pop.pop(); // Delete last "char", and return it or none.
    let p2 = string_pop.pop();
    if p1 != None {
        println!("{:?}", p1);
    }
    if p2!= None {
        println!("{:?}", p2);
    }
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn remove_string() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove occupies {} Bytes",
        std::mem::size_of_val(string_remove.as_str())
    );
    // Remove first chinese character.
    let c = string_remove.remove(0); // Delete a "char" with specific index, and return it.
    // Error, wrong chinese character start index.
    // string_remove.remove(1);

    // Remove second chinese character.
    // string_remove.remove(3);
    
    // dbg!(string_remove);

    // let d = string_remove.remove(234); // No index check or "Option", it's unsafe, be careful.

    // And clean string.
    string_remove.clear();
}

fn concatenate_string_with_add() {
    let string_append = String::from("hello "); 
    let string_rust = String::from("rust"); 
    
    // Added string should be a Slice or &str.
    // &string_rust will automatically deconstruct into &str
    /////////////////////////////////////////////////////////////////////////////////////////////////////
    let result = string_append + &string_rust; // It's not clone!!!!!  Value moved here.    /////
    /////////////////////////////////////////////////////////////////////////////////////////////////////
    // Reffer to it as "let result = string_append.add(&string_rust);
    // Ownership of string_append move to add(), then add() release it.

    // Shadow occurs here
    let mut result = result + "!"; // `result` in `result + "!"` is immutable.
    result += "!!!"; 

    println!("concatenate_string + -> {}", result);
}

fn concatenate_string_with_format() {
    let hello = String::from("Hello");
    let world = "world!";
    let new_str = format!("{}, {}", hello, world);

    println!("{}", new_str);
}
fn main() {
    // int_slice();

    // chinese_clice();

    // push_string();

    // insert_string();

    // replace_string();

    // pop_string();

    // remove_string();

    //concatenate_string_with_add();

    concatenate_string_with_format();
}
