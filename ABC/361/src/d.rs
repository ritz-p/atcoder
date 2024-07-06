use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut t: Chars,
    }
    let sb_count = s.iter().filter(|c| **c == 'B').count();
    let sw_count = s.iter().filter(|c| **c == 'W').count();
    let tb_count = t.iter().filter(|c| **c == 'B').count();
    let tw_count = t.iter().filter(|c| **c == 'W').count();

    if sb_count != tb_count || sw_count != tw_count {
        println!("-1");
        return;
    }
    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    s.push('.');
    s.push('.');
    t.push('.');
    t.push('.');

    queue.push_back(s.clone());
    map.insert(s.clone(), 0);
    while let Some(cs) = queue.pop_front() {
        for i in 0..n + 1 {
            let count = map[&cs];
            let empty = cs.iter().position(|c| *c == '.').unwrap();
            if cs[i] == '.' || cs[i + 1] == '.' {
                continue;
            }
            let mut new = cs.clone();
            new.swap(empty, i);
            new.swap(empty + 1, i + 1);

            if map.contains_key(&new) {
                continue;
            }
            map.insert(new.clone(), count + 1);
            queue.push_back(new);
        }
    }
    if let Some(res) = map.get(&t) {
        println!("{}", res);
    } else {
        println!("-1");
    }
}
