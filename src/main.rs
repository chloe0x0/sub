use sub_rs::*;

// 4bit even/odd check using IEEE-754
fn check_parity(a: Fbit8) -> Fbit {
    // check the lsb
    not(a[0])
}

fn main() {
    let x: u8 = 42;

    let even = check_parity(to_fbit8(x));
    if to_bool(even) {
        println!("{} is even according to IEEE-754 subtraction!", x)   
    } else {
        println!("{} is odd according to IEEE-754 subtraction!", x)   
    }
}
