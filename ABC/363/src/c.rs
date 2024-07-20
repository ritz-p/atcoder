use itertools::Itertools;
use std::collections::HashSet;
use proconio::input;


fn main() {
    input!{
        _n: usize,
        k: usize,
        s: String
    };
    let mut set = HashSet::new();
    for perm in s.chars().permutations(s.len()){
        let ps: String = perm.iter().collect();
        set.insert(ps);
    }
    let res = set.into_iter().filter(|p| sub(p,k)).count();
    println!("{}", res);
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn sub(s: &str, k: usize) -> bool {
    for i in 0..=s.len() - k {
        if is_palindrome(&s[i..i + k]) {
            return false;
        }
    }
    true
}