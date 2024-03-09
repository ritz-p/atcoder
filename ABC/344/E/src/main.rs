use proconio::input;
use std::collections::HashMap;
fn main(){
    input!{
        n: usize,
        a: [usize;n],
        q: usize,
    };
    let mut hash = HashMap::new();
    let mut rev_hash = HashMap::new();
    hash.insert(a[0],0);
    rev_hash.insert(0,a[0]);
    for i in 0..n-1{
        hash.insert(a[i+1],a[i]);
        rev_hash.insert(a[i],a[i+1]);
    }
    hash.insert(usize::MAX,a[a.len()-1]);
    rev_hash.insert(a[a.len()-1],usize::MAX);
    for i in 0..q{
        input!{
            c: usize,
        }
        match c{
            1 => {

            },
            2 => {

            },
            _ => {}
        }
        if c == 1{
            input!{
                x: usize,
                y: usize,
            };
            let r = *rev_hash.get(&x).unwrap();
            rev_hash.insert(x,y);
            rev_hash.insert(y,r);
            hash.insert(y,x);
            hash.insert(r,y);
        }else if c == 2{
            input!{
                x: usize,
            };

            let r = rev_hash.remove(&x);
            let h = hash.remove(&x);
            if let Some(entry) = rev_hash.get_mut(&h.unwrap()){
                *entry = r.unwrap();
            }
            if let Some(entry) = hash.get_mut(&r.unwrap()){
                *entry = h.unwrap();
            }
        }
    }
    let mut pos:usize = *rev_hash.get(&0).unwrap();
    loop{
        if pos != usize::MAX{
            print!("{} ",pos);
        }
        if let Some(h) = rev_hash.get(&pos){
            pos = *rev_hash.get(&pos).unwrap();
        }else{
            break;
        }
    }
}
