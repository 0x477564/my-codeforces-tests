/*
    Implementation - http://codeforces.com/problemset/problem/705/A
    Completed: 20/6/20 14:00
*/

use std::io;

fn main() {

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("error");

    let x: i32 = x.trim().parse().expect("error");

    if x <= 1 || x <= 100 {

        let mut boolean = false;

        for _x in 0..x - 1 {
            if boolean {
                print!("I love that ");
                boolean = false;
            } else {
                print!("I hate that ");
                boolean = true;
            }
        }

        if boolean {
            print!("I love it ");
        } else {
            print!("I hate it ");
        }

        println!("");
    }

}
