//! TBD

use super::errors::RingError;
use crate::arithmetic::engine::ext_gcd::extended_gcd;

pub struct Ring {
    modulo: u64,
}

pub struct RingItem(u64);

impl RingItem {
    pub fn value(&self) -> u64 {
        self.0
    }
}

// TODO: think about generalizing u64 into T
pub trait ModularInverse {
    type Error;

    /// Finds a multiplicative inverse modulo `modulus`.
    ///
    /// The result has three meanings:
    ///  - Ok(Some(x)): a _normalized_ inverse was found.
    ///  - Ok(None): the element has no inverse.
    ///  - Err(...): the algorithm failed or does not support this input.
    fn inverse(&self, item: RingItem, modulus: u64) -> Result<Option<u64>, Self::Error>;
}

impl Ring {
    pub fn new(modulo: u64) -> Result<Self, RingError> {
        if modulo == 0 {
            return Err(RingError::ZeroModulo);
        }

        Ok(Self { modulo })
    }

    pub fn new_item(&self, value: u64) -> RingItem {
        RingItem(value % self.modulo)
    }

    pub fn norm_item(&self, value: RingItem) -> RingItem {
        RingItem(value.0 % self.modulo)
    }

    pub fn add(&self, lhs: RingItem, rhs: RingItem) -> RingItem {
        let sum = u128::from(lhs.0) + u128::from(rhs.0);
        RingItem((sum % u128::from(self.modulo)) as u64)
    }

    pub fn neg(&self, item: RingItem) -> RingItem {
        let value = self.norm_item(item);
        if value.0 == 0 {
            return RingItem(0);
        }

        RingItem(self.modulo - value.0)
    }

    pub fn mul(&self, lhs: RingItem, rhs: RingItem) -> RingItem {
        let product = u128::from(lhs.0) * u128::from(rhs.0);
        RingItem((product % u128::from(self.modulo)) as u64)
    }

    #[allow(clippy::redundant_closure)] // Needed to avoid: aeneas > [Error] Name clash detected: the following identifiers are bound to the same name "arithmetic.modulo.RingItem" (the generated code will be incorrect)...
    pub fn mul_inverse<A: ModularInverse>(
        &self,
        item: RingItem,
        algo: &A,
    ) -> Result<Option<RingItem>, A::Error> {
        algo.inverse(self.norm_item(item), self.modulo)
            .map(|inverse| inverse.map(|value| RingItem(value)))
    }

    /// For ax = b (mod m)
    ///
    /// Returns:
    ///
    /// None
    ///     No solution exists.
    ///
    /// Some((first, step, count))
    ///     All solutions are first + k * step,
    ///     for k = 0, 1, ..., count - 1.
    pub fn solve_linear_congruence(
        &self,
        mut a: RingItem,
        mut b: RingItem,
    ) -> Result<Option<(RingItem, u64, u64)>, RingError> {
        a = self.norm_item(a);
        b = self.norm_item(b);

        let (g, _, s) = extended_gcd(self.modulo, a.0);
        if !b.0.is_multiple_of(g) {
            return Ok(None);
        }

        let step = self.modulo / g;

        let s_normalized = s.rem_euclid(i128::from(step)) as u64;
        let first = (u128::from(s_normalized) * u128::from(b.0 / g)) % u128::from(step);

        Ok(Some((self.new_item(first as u64), step, g)))
    }
}

#[cfg(test)]
mod tests {
    use crate::arithmetic::modulo::{Ring, RingItem};

    #[test]
    fn solve_single() {
        let cases = [
            (4, 2, 6, Some((2, 3, 2))),
            (17, 1, 43, Some((38, 43, 1))),
            (4, 1, 6, None),
            (0, 0, 6, Some((0, 1, 6))),
            (0, 1, 6, None),
            (0, 0, 1, Some((0, 1, 1))),
            (10, 8, 6, Some((2, 3, 2))),
            (2, u64::MAX - 1, u64::MAX, Some((u64::MAX / 2, u64::MAX, 1))),
        ];

        for (a, b, modulo, solution) in cases {
            let ring = Ring::new(modulo).unwrap();
            let actual = ring
                .solve_linear_congruence(RingItem(a), RingItem(b))
                .unwrap()
                .map(|(first, step, count)| (first.value(), step, count));
            assert_eq!(solution, actual, "a: {a}, b: {b}, modulus: {modulo}");
        }
    }
}
