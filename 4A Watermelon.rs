/*
    Brute force, math - http://codeforces.com/problemset/problem/4/A
    Completed: 2/5/20 21:15
*/

use std::io;

fn main() {

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("error");

    let x: i32 = x.trim().parse().expect("error");

    if x <= 1 || x <= 100 {

        let res = x % 2;

        if res == 0 && x != 2 {
            println!("YES");
        } else {
            println!("NO");
        }
    }

}
