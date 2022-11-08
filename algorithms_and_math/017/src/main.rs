use proconio::input;

fn main(){
    input!{
        n:usize,
        mut arr:[usize;n],
    };
    let mut res = 1;

    for i in 0..n{
        let div = get_common_divider(res,arr[i]);

        res = res / div * arr[i];
    }
    println!("{}",res);
}
//ユークリッド互除法
fn get_common_divider(mut a:usize,mut b:usize) -> usize{
    let mut zero = false;
    while !zero{
        if a <= b{
            b %= a;
        }else{
            a %= b;
        }
        if a == 0 || b == 0{
            zero = true;
        }
    }
    if a <= b{
        b
    }else{
        a
    }
}