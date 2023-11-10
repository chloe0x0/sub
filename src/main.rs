use sub_rs::*;

// 4bit even/odd check using IEEE-754
fn is_even(a: Fbit8) -> Fbit {
    // check the lsb
    not(a[0])
}

fn main() {
    let a: u8 = 6;
    let b: u8 = 3;

    let (g,e,l) = mag_comp4(
        to_fbit4(a), to_fbit4(b)
    );

    println!("{} | {} | {}", to_bool(g), to_bool(e), to_bool(l));

    if to_bool(g) {
        println!("{} > {}", a, b);
    } else if to_bool(e) {
        println!("{} = {}", a, b);
    } else if to_bool(l) {
        println!("{} < {}", a, b);
    } else {
        println!("Something fucked happened");
    }
}
