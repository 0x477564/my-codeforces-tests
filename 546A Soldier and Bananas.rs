/*
    Brute force, implementation, math - https://codeforces.com/problemset/problem/546/A
    Completed: 2/7/20 21:20
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let vector: Vec<&str> = input.trim().split(" ").collect();

    let bananas = &vector[2].parse::<i32>().unwrap();
    let dollars = &vector[1].parse::<i32>().unwrap();
    let initial_cost = &vector[0].parse::<i32>().unwrap();
    let mut current_banana = 0;
    let mut final_cost = 0;
    let mut borrow;

    for _ in 0..*bananas {
        current_banana += 1;
        final_cost += initial_cost * current_banana;
    }

    borrow = *dollars - final_cost;

    if borrow < 0 {
        borrow = -borrow;
        println!("{}", borrow);
    } else {
        println!("0");
    }

}