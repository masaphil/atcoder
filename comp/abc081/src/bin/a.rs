use proconio::{fastout, input};

#[fastout]
fn main() {
    input!(one: String);
    println!("{}", one.chars().filter(|&c| c == '1').count());
}
