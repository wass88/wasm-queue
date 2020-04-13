extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::collections::VecDeque;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    return format!("Hello, {}!", name);
}

#[wasm_bindgen]
pub fn zigzag(n: i32) -> i32 {
    let mut vec = VecDeque::new();
    let mut seed = 12345;
    let mut size = 0;
    let mut last = -1;
    for _ in 0..n {
        if size > 0 && (seed & 3) == 0 {
            last = vec.pop_front().unwrap();
            size -= 1;
        } else {
            vec.push_back(seed);
            size += 1;
        }
        seed = (seed * 1664525 + 1013904223) & 2147483647;
    }
    return last;
}
