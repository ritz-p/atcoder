use proconio::input;

fn main(){
    input!{
        n:usize,
        arr:[usize;n],
    };
    
    let mut arr_max:[usize;100000] = [0;100000];

    for i in 0..n{
        arr_max[arr[i]] += 1;
    }

    let mut res = 0;
    for i in 1..50000{
        res += arr_max[i] * arr_max[100000-i];
    }
    res += arr_max[50000] * (arr_max[50000] - 1) / 2;
    println!("{}",res);
}