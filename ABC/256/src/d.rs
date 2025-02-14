use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize,usize);n]
    };

    let mut sums = vec![0; 200001];
    for (l, r) in lr {
        sums[l] += 1;
        sums[r] -= 1;
    }
    for i in 1..200001 {
        sums[i] += sums[i - 1];
    }
    let mut left = 1;
    let mut right = 1;

    while right <= 200000 {
        if sums[right] > 0 && sums[left] > 0 {
            right += 1;
            continue;
        }

        if sums[right] == 0 && sums[left] > 0 {
            println!("{} {}", left, right);
            right += 1;
            left = right;
            continue;
        }

        if sums[right] == 0 && sums[left] == 0 {
            right += 1;
            left += 1;
        }
    }
}
