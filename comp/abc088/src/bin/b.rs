fn main() {
    proconio::input! {
    n: usize,
    mut v: [i32; n],
    }
    let a = v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", a);
}
