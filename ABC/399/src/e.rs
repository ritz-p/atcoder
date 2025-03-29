use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let mut v = vec!['?'; 26];
    let mut used = vec![false; 26];

    for i in 0..n {
        let sc = s[i];
        let tc = t[i];
        let si = (sc as u8 - b'a') as usize;
        used[si] = true;
        if v[si] == '?' {
            v[si] = tc;
        } else {
            if v[si] != tc {
                println!("-1");
                return;
            }
        }
    }
    for i in 0..26 {
        if !used[i] {
            v[i] = (b'a' + i as u8) as char;
        }
    }

    let mut count = 0;
    for i in 0..26 {
        if v[i] != (b'a' + i as u8) as char && used[i] {
            count += 1;
        }
    }
    let mut add = 0;
    let mut visited = vec![false; 26];
    for i in 0..26 {
        if !used[i] || v[i] == (b'a' + i as u8) as char || visited[i] {
            continue;
        }
        let mut current = i;
        let mut flag = false;
        while !visited[current] {
            visited[current] = true;
            let next = (v[current] as u8 - b'a') as usize;
            if next == i {
                flag = true;
                break;
            }
            if visited[next] {
                break;
            }
            current = next;
        }
        if flag {
            add += 1;
        }
    }

    let res = count + add;
    println!("{}", res);
}
