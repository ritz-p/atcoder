use proconio::input;

fn main(){
    input!{
        arr: [usize;64],
    };
    let mut res = 0;
    let mut mul:u64 = 1;
    if arr[0] == 1{
        res += 1;
    }
    for i in 1..64{
        mul = mul*2;
        if arr[i] == 1{
            res += mul;
        }
    }
    println!("{}",res);
}
