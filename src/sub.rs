
// Since we are using IEEE-754, our 'bits' are actually floating point numbers
pub type Fbit = f32;
pub type Fbit8 = [Fbit;8];
pub type Fbit4 = [Fbit;4];
// Define our true and false
pub const FALSE: Fbit = -0.0;
pub const TRUE: Fbit = 0.0;
// -0.0 - 0.0 = -0.0, -0.0 - -0.0 = -0.0 (signs agree)
pub fn not(a: Fbit) -> Fbit { FALSE - a }
pub fn or(a: Fbit, b: Fbit) -> Fbit { a - not(b) }
pub fn or4(a: Fbit, b: Fbit, c: Fbit, d: Fbit) -> Fbit {
    or(or(a,b), or(c,d))
}
pub fn nor(a: Fbit, b: Fbit) -> Fbit { not(or(a,b)) }
// AND is constructed using De-Morgan's Law: (a ∧ b = ⌐(⌐x ∨ ⌐y))
pub fn and(a: Fbit, b: Fbit) -> Fbit { not(or(not(a), not(b))) }
pub fn and4(a: Fbit, b: Fbit, c: Fbit, d: Fbit) -> Fbit {
    and(and(a, b), and(c, d))
}
pub fn nand(a: Fbit, b: Fbit) -> Fbit { not(and(a, b)) }
pub fn xor(a: Fbit, b: Fbit) -> Fbit { or(and(not(a), b), and(a, not(b))) }
pub fn xnor(a: Fbit, b: Fbit) -> Fbit { not(xor(a,b)) }
// Convert a u8 to an array of Fbits 
pub fn to_fbit8(x: u8) -> Fbit8 {
    let mut out: Fbit8 = [FALSE;8];

    for i in 0..8 {
        out[i] = if (x >> i) & 1 == 1 { TRUE } else { FALSE };
    }

    out
}
// Convert an array of Fbits to a u8 (from the blogpost)
pub fn from_fbit8(x: Fbit8) -> u8 {
    (0..8).filter(|i| x[*i].signum() > 0.0).map(|i| 1 << i).sum()
}

pub fn to_fbit4(x: u8) -> Fbit4 {
    if x > 16 {
        panic!();
    }

    let mut out: Fbit4 = [FALSE;4];

    for i in 0..4 {
        out[i] = if (x >> i) & 1 == 1 { TRUE } else { FALSE };
    }

    out
}

pub fn from_fbit4(x: Fbit4) -> u8 {
    (0..4).filter(|i| x[*i].signum() > 0.0).map(|i| 1 << i).sum()
}

// macro for checking equality of Fbit types
// this is needed since -0.0 == 0.0
#[macro_export]
macro_rules! fbit_eq {
    ($a: expr, $b: expr) => {
        $a.signum() == $b.signum()
    };
}
pub fn to_bool(a: Fbit) -> bool { a.signum() > 0.0 }


/*
    A 4-bit magnitude comparator using IEEE-754 Subtraction
*/

pub fn mag_comp4(a: Fbit4, b: Fbit4) -> (Fbit, Fbit, Fbit) {
    let equals: Fbit = and4(
        xnor(a[0], b[0]),
        xnor(a[1], b[1]),
        xnor(a[2], b[2]),
        xnor(a[3], b[3]),
    );

    let a_greater: Fbit = or4(
        and(not(b[3]), a[3]),
        and(and(a[2], not(b[2])), xnor(a[3], b[3])),
        and4(a[1], not(b[1]), xnor(a[3], b[3]), xnor(a[2], b[2])),
        and(and4(a[0], not(b[0]), xnor(a[3], b[3]), xnor(a[2], b[2])), xnor(a[1], b[1]))
    );

    let a_lesser: Fbit = or4(
        and(b[3], not(a[3])),
        and(and(not(a[2]), b[2]), xnor(a[3], b[3])),
        and4(not(a[1]), b[1], xnor(a[3], b[3]), xnor(a[2], b[2])),
        and(and4(not(a[0]), b[0], xnor(a[3], b[3]), xnor(a[2], b[2])), xnor(a[1], b[1]))
    );

    (a_greater, equals, a_lesser)
}

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