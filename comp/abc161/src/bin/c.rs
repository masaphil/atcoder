use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize
    };
    let dis: usize = n % k;
    println!("{}", std::cmp::min(dis, k - dis));
}
