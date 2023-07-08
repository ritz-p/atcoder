use proconio::input;

fn main(){
    input!{
        n: usize,
        k: usize,
        mut tup: [(usize,usize);n]
    };

    let mut max = 0;

    for i in 0..n{
        max += tup[i].1;
    }

    tup.sort_by(|a,b| a.0.partial_cmp(&(b.0)).unwrap());
    let mut day = 1;
    for i in 0..n{
        if max <= k{
            break;
        }
        max -= tup[i].1;
        day = tup[i].0+1;
    }
    println!("{}",day);
}
