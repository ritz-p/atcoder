use std::collections::HashMap;
 
use proconio::input;
use superslice::*;
fn main() {
    input!{
        _w: usize,
        _h: usize,
        n: usize,
        vpq: [(u64,u64);n],
        a: usize,
        va: [u64;a],
        b: usize,
        vb: [u64;b]
    };
    let mut cnts = HashMap::new();
    for (p,q) in vpq {
        let ia = va.lower_bound(&p);
        let ib = vb.lower_bound(&q);
        *cnts.entry((ia,ib)).or_insert(0) += 1u64;
    }
    let mut mx = 0;
    let mut mm = 1u64 << 60;
    for (_, &c) in cnts.iter() {
        mx = mx.max(c);
        mm = mm.min(c);
    }
    let mm = if cnts.len() == (a+1)*(b+1) {mm} else {0};
    println!("{} {}",mm, mx);
}