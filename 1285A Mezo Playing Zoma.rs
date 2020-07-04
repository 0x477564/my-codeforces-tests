/*
    Math - https://codeforces.com/problemset/problem/1285/A
    Completed: 4/7/20 16:18
*/

use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();

    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).unwrap();

    let mut left = 0;
    let mut right = 0;

    for char in second_input.trim().chars() {
        if char == 'L' {
            left -= 1;
        } else {
            right += 1;
        }
    }

    println!("{}", -left + right + 1);

}