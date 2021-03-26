use proconio::input;

fn main() {
    input! {
        mut a: [[u32;3];3],
        n:u32,
        b:[u32;n],
    };
    for fetch in b {
        //aの中身すべて取り出してbと確認
        //一緒だったら999に変える
        for i in 0..=2 {
            for j in 0..=2 {
                if a[i][j] == fetch {
                    a[i][j] = 999
                }
            }
        }
    }
    let mut ans: bool = false;
    if a[0][0] == 999 && a[0][1] == 999 && a[0][2] == 999 {
        ans = true
    }
    if a[1][0] == 999 && a[1][1] == 999 && a[1][2] == 999 {
        ans = true
    }
    if a[2][0] == 999 && a[2][1] == 999 && a[2][2] == 999 {
        ans = true
    }
    if a[0][0] == 999 && a[1][0] == 999 && a[2][0] == 999 {
        ans = true
    }
    if a[0][1] == 999 && a[1][1] == 999 && a[2][1] == 999 {
        ans = true
    }
    if a[0][2] == 999 && a[1][2] == 999 && a[2][2] == 999 {
        ans = true
    }
    if a[0][0] == 999 && a[1][1] == 999 && a[2][2] == 999 {
        ans = true
    }
    if a[0][2] == 999 && a[1][1] == 999 && a[2][0] == 999 {
        ans = true
    }
    if ans == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
