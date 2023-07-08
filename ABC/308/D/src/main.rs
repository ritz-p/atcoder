use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        a: [Chars;h]
    };

    let mut queue = VecDeque::new();
    let mut reached = vec![vec![false;w];h];

    if a[0][0] == 's'{
        queue.push_back((0,0));
        reached[0][0] = true;
    }

    let mut ans = false;
    let mut next = HashMap::new();
    next.insert('s','n');
    next.insert('n','u');
    next.insert('u','k');
    next.insert('k','e');
    next.insert('e','s');

    let dy = vec![-1,0,1,0];
    let dx = vec![0,1,0,-1];

    while let Some((y,x)) = queue.pop_front(){
        if y == h - 1 && x == w - 1{
            ans = true;
            break;
        }

        let nc = next[&a[y][x]];

        for r in 0..4{
            let ny = y as i64 + dy[r];
            let nx = x as i64 + dx[r];
            if ny < 0 || ny >= h as i64 || nx < 0 || nx >= w as i64{
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;

            if a[ny][nx] == nc{
                if !reached[ny][nx]{
                    queue.push_back((ny,nx));
                    reached[ny][nx] = true;
                }
            }
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
