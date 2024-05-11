use proconio::input;

fn main(){
    input!{
        n: usize,
        ss: [usize;n]
    };
    let mut res = 0;
    let mut m = 0;
    let mut s = 0;
    for i in ss.iter().rev(){
        res += i * m + s;
        let l = i.to_string().len();
    }
}
