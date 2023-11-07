
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
// macro for checking equality of Fbit types
// this is needed since -0.0 == 0.0
#[macro_export]
macro_rules! fbit_eq {
    ($a: expr, $b: expr) => {
        $a.signum() == $b.signum()
    };
}
