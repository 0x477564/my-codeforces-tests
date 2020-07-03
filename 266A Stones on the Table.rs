/*
    Implementation - https://codeforces.com/problemset/problem/266/A
    Completed: 3/7/20 2:05
*/

use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();

    let mut last_input = String::new();
    io::stdin().read_line(&mut last_input).unwrap();

    let mut byte: &u8 = &0;
    let mut out_value = 0;

    for letter in last_input.as_bytes() {
        if letter == byte {
            out_value += 1;
        }
        byte = letter;
    }

    println!("{}", out_value);
}