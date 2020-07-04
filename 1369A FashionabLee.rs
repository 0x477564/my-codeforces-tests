/*
    Geometry, math - https://codeforces.com/problemset/problem/1369/A
    Completed: 4/7/20 14:06
*/

use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();

    let first_input = first_input.trim().parse::<u16>().unwrap();

    for _ in 0..first_input {
        let mut another_input = String::new();
        io::stdin().read_line(&mut another_input).unwrap();

        let another_input = another_input.trim().parse::<u32>().unwrap();

        if another_input % 4 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }

}