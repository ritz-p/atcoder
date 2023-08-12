use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        m: usize,
        s: Chars,
        c: [usize;n]
    };
    let mut v = vec![vec![];m];

    for i in 0..n{
        v[c[i]-1].push(i);
    }
    for i in 0..m{
        let last = v[i].pop().unwrap();
        v[i].insert(0,last);
    }
    let mut pos = vec![0;m];
    for i in 0..n{
        print!("{}",s[v[c[i]-1][pos[c[i]-1]]]);
        pos[c[i]-1] += 1;
    }
    println!();
}
