use proconio::input;
use std::mem;

fn main() {
    input! {mut s:proconio::marker::Chars};
    for i in 0..s.len() {
        if s[i as usize] == 'B' && s[i + 1] == 'w' {
            mem::swap(&mut s[i], &mut s[i + 1])
        }
    }
}
