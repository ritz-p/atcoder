use proconio::input;

fn main(){
    input!{
        k: usize,
        g: usize,
        m: usize,
    };
    let mut glass = 0;
    let mut mug = 0;
    for _i in 0..k{
        if glass == g{
            glass = 0;
        }else if mug == 0 && glass != g{
            mug = m;
        }else{
            if glass + mug < g{
                glass += mug;
                mug = 0;
            }else if glass + mug > g{
                mug = glass + mug - g;
                glass = g;
            }else if glass + mug == g{
                glass = g;
                mug = 0;
            }
        }
        // println!("{} {}",glass,mug);

    }
    println!("{} {}",glass,mug);
}
