use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut arr: [usize;n],
    };
    let mut res = get_common_divider(arr[0],arr[1]);

    for i in 2..n{
        res = get_common_divider(res,arr[i]);
    }
    println!("{}",res);

}
fn get_common_divider(mut a:usize,mut b:usize) -> usize{
    let mut is_zero = false;
    while !is_zero {
        if a <= b {
            b %= a;
        } else {
            a %= b;
        }

        if a == 0 || b == 0 {
            is_zero = true
        }
    }
    if a <= b{
        b
    }else{
        a
    }

}