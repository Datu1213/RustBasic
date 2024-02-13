#![allow(unused)]

fn string_slice() {
    let mut str1: String = String::from("Hello, world");
    let str_slice1 = &str1[1..2]; // slice from index 1 to 2.
    let str_slice2 = &str1[ ..2]; // slice from start   to 2.
    let str_slice3 = &str1[1.. ]; // slice from index 1 to end.
    let str_slice3 = &str1[ .. ]; // slice from start   to end.
    // All slices above are immutable reference!!!
}

fn int_slice() {
    let array_int = [1, 2, 3, 4, 5]; // Do not use range;
    let array_slice = &array_int[1..];
    assert_eq!(array_slice, &[2, 3, 4, 5]);
}

fn ref_scope_and_var_scope() {
    let mut str = String::from("String");

    let str_ref = &str[2..]; // immutable ref "str_ref" start 

    str.clear(); // mutable ref "&str" start

    // println!("{}", str_ref); // immutable ref "str_ref" end

    // Dirty Data Warning. 
}

fn main() {
    int_slice();
}
