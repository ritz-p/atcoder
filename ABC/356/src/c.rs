use proconio::input;

fn main(){
    input!{
        n: usize,
        m: usize,
        k: usize,
    };
    let mut tests = vec![];
    for _i in 0..m{
        input!{
            c: usize,
            a: [usize;c],
            f: char,
        };
        tests.push((c,a,f));
    }
    let res = count(n,k,tests);
    println!("{}",res);
}

fn count(n: usize,k: usize, tests: Vec<(usize, Vec<usize>, char)>) -> usize {
    let mut total = 0;
    for mask in 0..(1 << n) {
        let mut is_valid = true;

        for (_c, a, f) in &tests {
            let current = a.iter().filter(|&&a| (mask & (1 << (a - 1))) != 0).count();

            if (*f == 'o' && current < k) || (*f == 'x' && current >= k) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            total += 1;
        }
    }

    total
}