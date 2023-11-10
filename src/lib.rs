
#[macro_use]
pub mod sub;
pub use sub::*;

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use crate::{sub::*, mag_comp4};

    #[test]
    fn test_not_table() {
        assert!(fbit_eq!(not(FALSE), TRUE));
        assert!(fbit_eq!(not(TRUE), FALSE));
    }

    #[test]
    fn test_or_table() {
        assert!(fbit_eq!(or(FALSE, FALSE), FALSE));
        assert!(fbit_eq!(or(TRUE, FALSE), TRUE));
        assert!(fbit_eq!(or(FALSE, TRUE), TRUE));
        assert!(fbit_eq!(or(TRUE, TRUE), TRUE));
    }

    #[test]
    fn test_nor_table() {
        assert!(fbit_eq!(nor(FALSE, FALSE), TRUE));
        assert!(fbit_eq!(nor(TRUE, FALSE), FALSE));
        assert!(fbit_eq!(nor(FALSE, TRUE), FALSE));
        assert!(fbit_eq!(nor(TRUE, TRUE), FALSE));
    }

    #[test]
    fn test_and_table() {
        assert!(fbit_eq!(and(FALSE, FALSE), FALSE));
        assert!(fbit_eq!(and(TRUE, FALSE), FALSE));
        assert!(fbit_eq!(and(FALSE, TRUE), FALSE));
        assert!(fbit_eq!(and(TRUE, TRUE), TRUE));
    }

    #[test]
    fn test_xor_table() {
        assert!(fbit_eq!(xor(FALSE, FALSE), FALSE));
        assert!(fbit_eq!(xor(TRUE, FALSE), TRUE));
        assert!(fbit_eq!(xor(FALSE, TRUE), TRUE));
        assert!(fbit_eq!(xor(TRUE, TRUE), FALSE));
    }

    #[test]
    fn test_xnor_table() {
        assert!(fbit_eq!(xnor(FALSE, FALSE), TRUE));
        assert!(fbit_eq!(xnor(TRUE, FALSE), FALSE));
        assert!(fbit_eq!(xnor(FALSE, TRUE), FALSE));
        assert!(fbit_eq!(xnor(TRUE, TRUE), TRUE));
    }

    #[test]
    fn test_nand_table() {
        assert!(fbit_eq!(nand(FALSE, FALSE), TRUE));
        assert!(fbit_eq!(nand(TRUE, FALSE), TRUE));
        assert!(fbit_eq!(nand(FALSE, TRUE), TRUE));
        assert!(fbit_eq!(nand(TRUE, TRUE), FALSE));
    }

    fn test_mag_case(a: u8, b: u8) {
        let (g,e,l) = mag_comp4(to_fbit4(a), to_fbit4(b));

        match a.cmp(&b) {
            Ordering::Less => { assert!(to_bool(l)) },
            Ordering::Equal => { assert!(to_bool(e)) },
            Ordering::Greater => { assert!(to_bool(g)) }
        }
    }

    #[test]
    fn test_mag() {
        test_mag_case(0, 1);
        test_mag_case(1, 0);
        test_mag_case(2, 0);
        test_mag_case(3, 2);
        test_mag_case(10, 3);
        test_mag_case(3, 6);
    }
}
