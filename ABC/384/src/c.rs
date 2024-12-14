use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    };
    let mut bmap = BTreeMap::new();
    let list = vec![
        "ABCDE", "BCDE", "ACDE", "ABDE", "ABCE", "ABCD", "CDE", "BDE", "ADE", "BCE", "ACE", "BCD",
        "ABE", "ACD", "ABD", "ABC", "DE", "CE", "BE", "CD", "AE", "BD", "AD", "BC", "AC", "AB",
        "E", "D", "C", "B", "A",
    ];
    for s in list {
        let mut current = 0;
        for j in s.chars() {
            match j {
                'A' => {
                    current += a;
                }
                'B' => {
                    current += b;
                }
                'C' => {
                    current += c;
                }
                'D' => {
                    current += d;
                }
                'E' => {
                    current += e;
                }
                _ => {}
            }
        }
        bmap.entry(current).or_insert(vec![]).push(s);
    }
    for (_k, mut v) in bmap.into_iter().rev() {
        v.sort();
        for s in v {
            println!("{}", s);
        }
    }
}
