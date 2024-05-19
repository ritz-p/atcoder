use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut list = vec![];

    for bits in 2..(1 << 10) {
        let mut num: i64 = 0;

        for j in (0..10).rev() {
            if bits >> j & 1 == 1 {
                println!("{}",bits >> j);
                num = num * 10 + j;
            }
        }

        list.push(num);
    }

    list.sort();

    println!("{}", list[k - 1]);
}
