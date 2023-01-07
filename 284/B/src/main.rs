use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    let mut res = vec![];
    for _i in 0..n{
        input!{
            m: usize,
            arr: [usize;m],
        };
        let mut odd = 0;
        for j in 0..m{
            if arr[j] % 2 == 1{
                odd += 1;
            }
        }
        res.push(odd);
    }
    for i in 0..res.len(){
        println!("{}",res[i]);
    }
}