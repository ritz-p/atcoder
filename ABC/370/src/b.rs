use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut elements = vec![];

    for i in 1..=n {
        input! {
            a: [usize;i]
        };
        elements.push(a);
    }
    let mut current = elements[0][0];
    for i in 2..=n {
        if current >= i {
            current = elements[current - 1][i - 1];
        } else {
            current = elements[i - 1][current - 1];
        }
    }
    println!("{}", current);
}
