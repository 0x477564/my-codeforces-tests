/*
    Implementation, strings - https://codeforces.com/problemset/problem/734/A
    Completed: 4/7/20 12:06
*/

use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();

    let mut last_input = String::new();
    io::stdin().read_line(&mut last_input).unwrap();

    let mut anton = 0;
    let mut danik = 0;

    for char in last_input.trim().chars() {
        if char.eq(&'D') {
            danik += 1;
        } else {
            anton += 1;
        }
    }

    if anton == danik {
        println!("Friendship");
    } else if anton > danik {
        println!("Anton");
    } else {
        println!("Danik");
    }

}