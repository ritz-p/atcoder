use std::{
    collections::HashSet,
    io::{self, BufReader},
};

use proconio::{input, source::line::LineSource};

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize,
    };
    let mut set = HashSet::new();
    set.insert(1);
    println!("1");
    loop {
        input! {
            e: usize
        };
        if e == 0 {
            return;
        }
        set.insert(e);
        let mut current = e + 1;
        while set.contains(&current) || current >= 2 * (n + 1) {
            current += 1;
            if current >= 2 * (n + 1) {
                current = 1;
            }
        }
        set.insert(current);
        println!("{}", current);
    }
}
