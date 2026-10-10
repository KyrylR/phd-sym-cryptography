//! TBD

use super::errors::RingError;

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
}
