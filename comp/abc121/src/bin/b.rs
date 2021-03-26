use proconio::input;

fn main() {
    input!(
        n: usize,
        m: usize,
        c: isize,
        b: [isize; m],
        a: [[isize; m]; n]
    );
    let mut ans = 0;
    for i in a {
        let mut sum = 0;

        for num in 0..m {
            sum += i[num] * b[num]
        }
        sum += c;
        if sum > 0 {
            ans += 1
        }
    }
    println!("{}", ans);
}
