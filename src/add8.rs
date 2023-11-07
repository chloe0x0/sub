pub use crate::*;

/*
    Implementation of an 8 bit adder using IEEE-754 Subtraction

*/

/// A one bit full adder
pub fn one_bit_full(a: Fbit, b: Fbit, c: Fbit) -> (Fbit, Fbit) {
    (
        xor(xor(a,b), c),
        or(
            and(xor(a,b),c),
            and(a,b)
        )
    )
}


/// A functional 8 bit adder using IEEE-754 subtraction
pub fn adder(a: Fbit8, b: Fbit8) -> Fbit8 {
    let (s0, c0) = one_bit_full(a[0], b[0], FALSE);
    let (s1, c1) = one_bit_full(a[1], b[1], c0);
    let (s2, c2) = one_bit_full(a[2], b[2], c1);
    let (s3, c3) = one_bit_full(a[3], b[3], c2);
    let (s4, c4) = one_bit_full(a[4], b[4], c3);
    let (s5, c5) = one_bit_full(a[5], b[5], c4);
    let (s6, c6) = one_bit_full(a[6], b[6], c5);
    let (s7, _)  = one_bit_full(a[7], b[7], c6);

    [s0, s1, s2, s3, s4, s5, s6, s7]
}
