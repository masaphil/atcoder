use proconio::input;

fn main() {
    input! {n:u32};
    let mut ans = 1;
    for i in 0..=n {
        for j in 0..8 {
            if i == 2u32.pow(j) {
                ans = i;
            };
        }
    }
    println!("{}", ans)
}
