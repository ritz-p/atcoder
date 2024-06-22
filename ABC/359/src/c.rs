use proconio::input;

fn main(){
    input!{
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize
    };
    let y_diff = (sy-ty).abs();
    let mut res = y_diff;
    let x_diff = (sx-tx).abs();
    let mut ignore = 0;
    if sy % 2 == 0{
        if (sx % 2 == 0 && sx < tx) || (sx % 2 == 1 && sx > tx){
            ignore = y_diff + 1;
        }else{
            ignore = y_diff;
        }
    }else{
        if (sx % 2 == 0 && sx > tx) || (sx % 2 == 1 && sx < tx){
            ignore = y_diff + 1;
        }else{
            ignore = y_diff;
        }
    }
    if x_diff > ignore{
        res += (x_diff - ignore + 1) / 2;
    }
    println!("{}",res);
}
