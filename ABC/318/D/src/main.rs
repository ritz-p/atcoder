use proconio::input;

fn main(){
    input!{
        n: usize
    };
    let mut dp = vec![0;1<<n];
    for i in 0..n-1{
        input!{
            d: [i64; n-1-i]
        }
        for (j,&d) in d.iter().enumerate(){
            let j = j+i+1;
            let mut next = dp.clone();
            for (bit,dp) in dp.iter().enumerate(){
                if bit >> i & 1 == 0 && bit >> j & 1 == 0{
                    let x = bit | ( 1 << i) | (1 << j);
                    next[x] = next[x].max(*dp + d);
                }
            }
            dp = next;
        }
    }
    let res = dp.into_iter().max().unwrap();
    println!("{}",res);
}
