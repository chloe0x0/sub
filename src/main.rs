use sub_rs::*;

fn main() {
    let a = to_fbit8(9);
    let b = to_fbit8(10);
    println!("{} + {} = {}", 9, 10, from_fbit8(adder(a, b)));
}
