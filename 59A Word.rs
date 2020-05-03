/*
    Implementations, strings - http://codeforces.com/problemset/problem/59/A
    Completed: 2/5/20 22:40
*/

use std::io;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("error");

    if x.len() >= 1 || x.len() <= 100 {

        let mut _z = 0;
        let mut _upper = 0;
        let mut _down = 0;

        for y in x.trim().as_bytes() {
            for z in LETTERS.as_bytes() {
                if z.eq_ignore_ascii_case(&y) {
                    _z += 1;
                }
            }

            if _z > 0 {
                _z = 0;
            } else {
                panic!("phrase contains non-latin characters");
            }

            if y.is_ascii_uppercase() {
                _upper += 1;
            } else if y.is_ascii_lowercase() {
                _down += 1;
            }
        }

        if _upper < _down || _upper == _down {
            println!("{}", x.to_lowercase());
        } else {
            println!("{}", x.to_uppercase());
        }
    }

}
