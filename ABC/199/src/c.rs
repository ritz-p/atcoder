use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        mut s: Chars,
        q: usize,
        tab: [(usize,usize,usize);q]
    };
    let mut reversed = false;
    for (t,a,b) in tab{
        match t{
            1 => {
                if !reversed{
                    s.swap(a-1, b-1);
                }else{
                    s.swap((a-1+n)%(2*n),(b-1+n)%(2*n));
                }
            },
            2 => {
                reversed = !reversed;
            },
            _ => {}
        }
    }
    if reversed{
        let mut front = s[0..n].to_vec();
        let back = s[n..].to_vec();
        s = back;
        s.append(&mut front);
    }
    println!("{}",s.iter().join(""));
}
