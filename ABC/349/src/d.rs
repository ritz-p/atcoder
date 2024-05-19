use proconio::input;

fn main(){
    input!{
        mut l: usize,
        r: usize,
    };

    let mut res:Vec<(usize,usize)> = vec![];

    loop{
        if l == r{
            break;
        }
        let mut coe: u32 = 0;
        while l % 2usize.pow(coe+1) == 0 && l + 2usize.pow(coe+1) <= r{
            coe += 1;
        }
        res.push((l,l+2usize.pow(coe)));
        l += 2usize.pow(coe);
    }
    println!("{}",res.len());
    for (x,y) in res{
        println!("{} {}",x,y);
    }
}
