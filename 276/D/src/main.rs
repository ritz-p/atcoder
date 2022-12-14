use proconio::input;

fn main(){
    input!{
        n: usize,
        mut arr: [usize;n],
    };
    let mut res = 0;
    let mut gcd = 0;
    for i in 0..n{
        gcd = get_common_divider(gcd,arr[i]);
    }
    
    for i in 0..n{
        arr[i] /= gcd;
        while arr[i] % 2==0{
            arr[i] /= 2;
            res += 1;
        }
        while arr[i] % 3==0{
            arr[i] /= 3;
            res += 1;   
        }
        if arr[i] > 1{
            res = -1;
            break;
        }
    }
    println!("{}",res);
}

fn get_common_divider(mut a:usize,mut b:usize) -> usize{
    let mut zero = false;
    if a == 0 || b == 0{
        zero = true;
    }
    // println!("{},{}",a,b);
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