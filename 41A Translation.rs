/*
    Implementation, strings - http://codeforces.com/problemset/problem/41/A
    Completed: 2/7/20 16:15
*/

use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input);

    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input);

    first_input = first_input.trim().parse().unwrap();
    second_input = second_input.trim().parse().unwrap();

    let second_input: String = second_input.chars().rev().collect();

    if first_input.eq(&second_input) {
        println!("YES");
    } else {
        println!("NO");
    }
}