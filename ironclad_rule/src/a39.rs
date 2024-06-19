use proconio::input;

fn main(){
    input!{
        n: usize,
        mut lr: [(usize,usize);n]
    };
    let mut res = 0;
    let mut current = 0;

    lr.sort_by(|a,b|a.1.cmp(&b.1));
    for (l,r) in lr{
        if current <= l{
            res += 1;
            current = r;
        }
    }
    println!("{}",res);
}