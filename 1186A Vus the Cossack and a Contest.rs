/*
    Implementation - https://codeforces.com/problemset/problem/1186/A
    Completed: 4/7/20 12:55
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let vector: Vec<&str> = input.trim().split(" ").collect();

    let participants = vector[0].parse::<u16>().unwrap();
    let pens  = vector[1].parse::<u16>().unwrap();
    let notebooks  = vector[2].parse::<u16>().unwrap();

    if (participants <= pens) & (participants <= notebooks) {
        println!("Yes");
    } else {
        println!("No");
    }

}