use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n]
    };
    let mut res = 0;

    for i in 0..n{
        for j in i+1..n{
            for k in j+1..n{
                for l in k+1..n{
                    for m in l+1..n{
                        if arr[i] + arr[j] + arr[k] + arr[l] + arr[m] == 1000{
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}",res);
}