use proconio::input;
use std::collections::HashSet;
use std::iter::FromIterator;
fn main(){
    input!{
        n: usize,
        m: usize,
    };
    let mut arr:Vec<(usize,usize,HashSet<usize>)> = vec![];
    for _i in 0..n{
        input!{
            p: usize,
            c: usize,
            f: [usize;c],
        };
        let set: HashSet<usize> = HashSet::from_iter(f.iter().cloned());
        arr.push((p,c,set));
    }
    for i in 0..n{
        for j in 0..n{
            if i == j{
                continue
            }
            if arr[i].0 == arr[j].0 && arr[i].1 == arr[j].1{
                continue
            }
            let diff1:HashSet<usize> = arr[i].2.difference(&arr[j].2).copied().collect();
            let diff2:HashSet<usize> = arr[j].2.difference(&arr[i].2).copied().collect();
            if arr[i].0 <= arr[j].0 && diff2.len() == 0{
                println!("Yes");
                return
            }
            if arr[i].0 >= arr[j].0 && diff1.len() == 0{
                println!("Yes");
                return
            }
        }
    }
    println!("No");
}
