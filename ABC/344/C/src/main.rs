use proconio::input;
use std::collections::HashSet;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut prog = vec![];
    input!{
        m: usize,
    };
    for _i in 0..m{
        input!{
            b: usize,
        }
        for j in 0..n{
            prog.push(a[j]+b);
        }
    }
    let mut res = HashSet::new();
    input!{
        l: usize
    };
    for _i in 0..l{
        input!{
            c: usize,
        }
        for j in prog.iter(){
            res.insert(c + j);
        }
    }
    input!{
        q: usize,
    }
    for _i in 0..q{
        input!{
            d: usize,
        };
        if res.contains(&d){
            println!("Yes");
        }else{
            println!("No");
        }
    }
}
