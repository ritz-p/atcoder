use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let mut ks = vec![];
    for i in 1..1000001{
        let k = i * i * i;
        let s = k.to_string();
        let s_rev = s.chars().rev().collect::<String>();
        if s == s_rev{
            ks.push(k);
        }
    }

    let mut left = 0;
    let mut right = ks.len()-1;
    while left <= right{
        let mid = (right + left) / 2;
        if ks[mid] < n{
            left = mid + 1;
        }else if ks[mid] > n{
            right = mid - 1;
        }else if ks[mid] == n{
            println!("{}",ks[mid]);
            return;
        }
    }
    println!("{}",ks[right]);
}
