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

fn chinese_clice() {
    let str: String = String::from("中国话");
    let str_ref = &str[0..1];
    println!("{}", str_ref); //Error, chinese character occupies 3 Bytes.
    // byte index 1 is not a char boundary; it is inside '中' (bytes 0..3) of `中国话`
}

fn main() {
    // int_slice();

    // chinese_clice();
}
