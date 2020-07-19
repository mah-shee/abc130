#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        w: f64,
        h: f64,
        x: f64,
        y: f64
    }
    let rec: f64 = w * h / 2.0;

    if x == w / 2.0 && y == h / 2.0 {
        println!("{} {}", rec, 1);
    } else {
        println!("{} {}", rec, 0);
    }
}
