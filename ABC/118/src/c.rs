use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut res = 0;
    for i in a{
        res = get_common_divider(res,i);
    }
    println!("{}",res);
}
fn get_common_divider(mut a:usize,mut b:usize) -> usize{
    let mut zero = false;
    if a == 0 || b == 0{
        zero = true;
    }
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
    a.max(b)
}