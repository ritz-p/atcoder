use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        n: usize,
        m: usize,
        s: [Chars;n]
    };
    let nmax = n-9;
    let mmax = m-9;
    let mut lo = true;
    for i in 0..=nmax{
        for j in 0..=mmax{
            lo = true;
            for k in 0..9{
                if !lo{
                    break;
                }
                for l in 0..9{
                    if k < 3 && l < 3{
                        if s[k+i][l+j] != '#'{
                            lo = false;
                            break;
                        }
                    }
                    if k >= 6 && l >= 6{
                        if s[k+i][l+j] != '#'{
                            lo = false;
                            break;
                        }
                    }
                    if  k <= 3 && l == 3{
                        if s[k+i][l+j] != '.'{
                            lo = false;
                            break;
                        }
                    }
                    if  l <= 3 && k == 3{
                        if s[k+i][l+j] != '.'{
                            lo = false;
                            break;
                        }
                    }
                    if k >= 5 && l == 5{
                        if s[k+i][l+j] != '.'{
                            lo = false;
                            break;
                        }
                    }
                    if l >= 5 && k == 5{
                        if s[k+i][l+j] != '.'{
                            lo = false;
                            break;
                        }
                    }
                    if l == 8 && k == 8{
                        println!("{} {}",i+1,j+1);
                    }
                }
            }
        }
    }
}
