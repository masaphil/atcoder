use proconio::input;

fn main() {
    input!(n: i64);
    let mut ans = String::from(":(");
    for i in 1..=50000 {
        if (i as f64 * 1.08).floor() as i64 == n {
            ans = i.to_string();
            println!("{}", ans)
        }
    }
    println!("{}", ans)
}
