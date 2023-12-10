use proconio::input;

fn main(){
    input!{
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize,usize);n]
    };
    let mut sum = 0;
    for i in 0..n{
        sum += pq[i].0 * pq[i].1;
    }
    if sum < s{
        sum += k; 
    }
    println!("{}",sum);
}
