
// Since we are using IEEE-754, out 'bits' are actually floating point numbers
pub type Fbit = f32;
pub type Fbit8 = [Fbit;8];
// Define out true and false
pub const FALSE: Fbit = -0.0;
pub const TRUE: Fbit = 0.0;
// -0.0 - 0.0 = -0.0, -0.0 - -0.0 = -0.0 (signs agree)
pub fn not(a: Fbit) -> Fbit { FALSE - a }
pub fn or(a: Fbit, b: Fbit) -> Fbit { a - not(b) }
pub fn nor(a: Fbit, b: Fbit) -> Fbit { not(or(a,b)) }
// AND is constructed using De-Morgan's Law: (a ∧ b = ⌐(⌐x ∨ ⌐y))
pub fn and(a: Fbit, b: Fbit) -> Fbit { not(or(not(a), not(b))) }
pub fn nand(a: Fbit, b: Fbit) -> Fbit { not(and(a, b)) }
pub fn xor(a: Fbit, b: Fbit) -> Fbit { or(and(not(a), b), and(a, not(b))) }
pub fn xnor(a: Fbit, b: Fbit) -> Fbit { not(xor(a,b)) }
// Convert a u8 to a array of Fbits 
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
// macro for checking equality of Fbit types
// this is needed since -0.0 == 0.0
#[macro_export]
macro_rules! fbit_eq {
    ($a: expr, $b: expr) => {
        $a.signum() == $b.signum()
    };
}
pub fn to_bool(a: Fbit) -> bool { a.signum() > 0.0 }
