use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        mut a: [usize;n],
        mut b: [usize;m]
    };

    a.sort();
    b.sort();

    let mut i = 0;
    let mut j = 0;
    loop{
        if a[i] <= b[j]{
            if (i+1) >= (m-j){
                println!("{}",a[i]);
                return;
            }
            if i == n-1{
                println!("{}",b[m-1]+1);
                return;
            }
            i += 1;
        }else{
            if (i+1) >= (m-j){
                println!("{}",b[j] + 1);
                return;
            }
            if j == m-1{
                println!("{}",a[i]);
                return;
            }
            j += 1;
        }
    }
}
