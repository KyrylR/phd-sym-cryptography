//! Implementation of the extended Euclidean algorithm for the [`crate::arithmetic::modulo::ModularInverse`]

use crate::arithmetic::errors::RingError;
use crate::arithmetic::modulo::{ModularInverse, RingItem};

pub struct ExtGcd;

impl ModularInverse for ExtGcd {
    type Error = RingError;

    fn inverse(&self, item: RingItem, modulus: u64) -> Result<Option<u64>, Self::Error> {
        if modulus == 0 {
            return Err(RingError::ZeroModulo);
        }

        let (gcd, _, coefficient) = extended_gcd(modulus, item.value() % modulus);
        if gcd != 1 {
            return Ok(None);
        }

        // Normalize the inverse to the [0, modulus) range
        Ok(Some(coefficient.rem_euclid(i128::from(modulus)) as u64))
    }
}

/// Accepts two non-negative integers a and b with a ≥ b.
///
/// Returns d = gcd(a, b) and integers x, y satisfying ax + by = d.
///
/// Source: Menezes, van Oorschot, and Vanstone, *Handbook of Applied
/// Cryptography*, Algorithm 2.107, p. 67.
/// [Book DOI](https://doi.org/10.1201/9780429466335) ·
/// [Authors' chapter PDF](https://cacr.uwaterloo.ca/hac/about/chap2.pdf#page=20).
fn extended_gcd(mut a: u64, mut b: u64) -> (u64, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }

    let mut x2 = 1_i128;
    let mut x1 = 0_i128;
    let mut y2 = 0_i128;
    let mut y1 = 1_i128;

    while b > 0 {
        let q = i128::from(a / b);
        let r = a % b;
        let x = x2 - q * x1;
        let y = y2 - q * y1;
        (a, b) = (b, r);
        (x2, x1) = (x1, x);
        (y2, y1) = (y1, y);
    }

    (a, x2, y2)
}

#[cfg(test)]
mod tests {
    use super::ExtGcd;
    use crate::arithmetic::errors::RingError;
    use crate::arithmetic::modulo::{ModularInverse, Ring};

    #[test]
    fn extended_gcd() {
        let cases: &[(u64, u64, (u64, i128, i128))] = &[
            (0, 0, (0, 1, 0)),
            (17, 0, (17, 1, 0)),
            (0, 17, (17, 0, 1)),
            (17, 17, (17, 0, 1)),
            (48, 18, (6, -1, 3)),
            (17, 43, (1, -5, 2)),
            (u64::MAX, u64::MAX - 1, (1, 1, -1)),
            (
                12_200_160_415_121_876_738,
                7_540_113_804_746_346_429,
                (1, -2_880_067_194_370_816_120, 4_660_046_610_375_530_309),
            ),
        ];

        for &(a, b, expected) in cases {
            assert_eq!(super::extended_gcd(a, b), expected, "inputs: ({a}, {b})");
        }
    }

    #[test]
    fn modular_inverse() {
        let cases: &[(u64, u64, Option<u64>)] = &[
            (0, 1, Some(0)),
            (0, 7, None),
            (17, 43, Some(38)),
            (60, 43, Some(38)),
            (5, 6, Some(5)),
            (2, 6, None),
            (2, u64::MAX, Some(1_u64 << 63)),
        ];

        for &(value, modulus, expected) in cases {
            let ring = Ring::new(modulus).unwrap();
            let actual = ExtGcd.inverse(ring.new_item(value), modulus).unwrap();
            assert_eq!(actual, expected, "value: {value}, modulus: {modulus}");
        }

        let zero = Ring::new(1).unwrap().new_item(0);
        assert!(matches!(
            ExtGcd.inverse(zero, 0),
            Err(RingError::ZeroModulo)
        ));
    }
}
