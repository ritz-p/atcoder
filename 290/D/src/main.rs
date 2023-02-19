use proconio::input;

fn main(){
    input!{
        t: usize,
    };
    let mut tests = vec![];

    for i in 0..t{
        input!{
            n: usize,
            d: usize,
            k: usize,
        };
        let test = vec![n,d,k];
        tests.push(test);
    }
    println!("{:?}",tests);

    for i in 0..t{
        let mut flags = vec![false,n];
        // 1の行程
        flags[0] = true;
        let mut last = 0;
        for j in 0..tests[i][2]{
            last = (last + tests[i][1]) % n;
            for l in 
        }
    }
}
