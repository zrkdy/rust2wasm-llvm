//use regex::Regex;
//extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// fn main() {
//     println!("Hello, app2");
// }

#[no_mangle]
#[wasm_bindgen]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    println!("hello test2");
    a + b
}