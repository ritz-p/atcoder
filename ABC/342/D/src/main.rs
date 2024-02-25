use proconio::input;
fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };
    let mut res = 0;
    let mut c = vec![0;200001];
    for x in a{
        let mut y = x;
        let mut i = 2;
        while i * i <= y{
            while y % (i * i) == 0{
                y /= i * i;
            }
            i += 1;
        }
        res += c[y];
        c[y] += 1;
    }
    res += c[0] * (n - c[0]);
    println!("{}",res);
}
