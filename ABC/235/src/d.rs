use std::collections::{HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        a: usize,
        n: String,
    };

    let mut queue = VecDeque::new();
    queue.push_back((String::from("1"), 0));
    let mut map = HashMap::new();

    while let Some((cs, res)) = queue.pop_front() {
        if cs == n {
            println!("{}", res);
            return;
        }
        let current = cs.parse::<usize>();

        if let Ok(v) = current {
            let next = v * a;
            if next.to_string().len() <= n.len() && !map.contains_key(&next.to_string()) {
                map.insert(next.to_string(), res + 1);
                queue.push_back((next.to_string(), res + 1));
            }

            if let Ok(cv) = current {
                if cv % 10 != 0 && n.len() >= 2 {
                    let mut s = vec![];
                    s.push(cs.chars().nth(cs.len() - 1).unwrap());
                    s.extend_from_slice(&cs[0..cs.len() - 1].chars().collect::<Vec<char>>());
                    if !map.contains_key(&s.iter().collect::<String>()) {
                        map.insert(s.iter().collect::<String>(), res + 1);
                        queue.push_back((s.iter().collect::<String>(), res + 1));
                    }
                }
            }
        }
    }

    println!("-1");
}
