
#[macro_use]
pub mod sub;
pub use sub::*;

pub mod add8;
pub use add8::*;

#[cfg(test)]
mod test {
    use crate::sub::*;

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
    fn test_nand_table() {
        assert!(fbit_eq!(nand(FALSE, FALSE), TRUE));
        assert!(fbit_eq!(nand(TRUE, FALSE), TRUE));
        assert!(fbit_eq!(nand(FALSE, TRUE), TRUE));
        assert!(fbit_eq!(nand(TRUE, TRUE), FALSE));
    }
}
