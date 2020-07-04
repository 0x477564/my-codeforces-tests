/*
    Implementation, strings - https://codeforces.com/problemset/problem/281/A
    Completed: 4/7/20 3:43
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut output = String::from("");
    let mut i = 0;

    for mut letter in input.trim().chars() {
        i += 1;
        if !letter.is_uppercase() & (i == 1) {
            letter.make_ascii_uppercase();
        }
        output.push(letter);
    }

    println!("{}", output);
}