use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign,
    Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// Represents a number in a Galois field
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GaloisField<T>(pub T);

impl From<u8> for GaloisField<u8> {
    fn from(a: u8) -> Self {
        Self(a)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for GaloisField<u8> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<u8> for GaloisField<u8> {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<GaloisField<Self>> for u8 {
    type Output = Self;

    fn add(self, rhs: GaloisField<Self>) -> Self::Output {
        self ^ rhs.0
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign for GaloisField<u8> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<u8> for GaloisField<u8> {
    fn add_assign(&mut self, rhs: u8) {
        self.0 ^= rhs;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<GaloisField<Self>> for u8 {
    fn add_assign(&mut self, rhs: GaloisField<Self>) {
        *self ^= rhs.0;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for GaloisField<u8> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub<u8> for GaloisField<u8> {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub<GaloisField<Self>> for u8 {
    type Output = Self;

    fn sub(self, rhs: GaloisField<Self>) -> Self::Output {
        self ^ rhs.0
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign for GaloisField<u8> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign<u8> for GaloisField<u8> {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 ^= rhs;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign<GaloisField<Self>> for u8 {
    fn sub_assign(&mut self, rhs: GaloisField<Self>) {
        *self ^= rhs.0;
    }
}

impl Mul for GaloisField<u8> {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let mut p = Self(0);

        for _ in 0..8 {
            p ^= ((((rhs & 1).0 as i8) << 7).wrapping_shr(7) as u8) & self;
            rhs >>= 1;
            let carry = (self >> 7) & 1;
            self <<= 1;
            self ^= ((((carry & 1).0 as i8) << 7).wrapping_shr(7) as u8) & 0x1B;
        }

        p
    }
}

impl Mul<u8> for GaloisField<u8> {
    type Output = Self;

    fn mul(mut self, mut rhs: u8) -> Self::Output {
        let mut p = Self(0);

        for _ in 0..8 {
            p ^= ((((rhs & 1) as i8) << 7).wrapping_shr(7) as u8) & self;
            rhs >>= 1;
            let carry = (self >> 7) & 1;
            self <<= 1;
            self ^= ((((carry & 1).0 as i8) << 7).wrapping_shr(7) as u8) & 0x1B;
        }

        p
    }
}

impl Mul<GaloisField<Self>> for u8 {
    type Output = Self;

    fn mul(mut self, mut rhs: GaloisField<Self>) -> Self::Output {
        let mut p = 0;

        for _ in 0..8 {
            p ^= ((((rhs & 1).0 as i8) << 7).wrapping_shr(7) as Self) & self;
            rhs >>= 1;
            let carry = (self >> 7) & 1;
            self <<= 1;
            self ^= ((((carry & 1) as i8) << 7).wrapping_shr(7) as Self) & 0x1B;
        }

        p
    }
}

impl MulAssign for GaloisField<u8> {
    fn mul_assign(&mut self, mut rhs: Self) {
        let mut p = Self(0);

        for _ in 0..8 {
            p ^= ((((rhs & 1).0 as i8) << 7).wrapping_shr(7) as u8) & *self;
            rhs >>= 1;
            let carry = (*self >> 7) & 1;
            *self <<= 1;
            *self ^= ((((carry & 1).0 as i8) << 7).wrapping_shr(7) as u8) & 0x1B;
        }

        *self = p;
    }
}

impl MulAssign<u8> for GaloisField<u8> {
    fn mul_assign(&mut self, mut rhs: u8) {
        let mut p = Self(0);

        for _ in 0..8 {
            p ^= ((((rhs & 1) as i8) << 7).wrapping_shr(7) as u8) & *self;
            rhs >>= 1;
            let carry = (*self >> 7) & 1;
            *self <<= 1;
            *self ^= ((((carry & 1).0 as i8) << 7).wrapping_shr(7) as u8) & 0x1B;
        }

        *self = p;
    }
}

impl MulAssign<GaloisField<Self>> for u8 {
    fn mul_assign(&mut self, mut rhs: GaloisField<Self>) {
        let mut p = GaloisField::<Self>(0);

        for _ in 0..8 {
            p ^= ((((rhs & 1).0 as i8) << 7).wrapping_shr(7) as Self) & *self;
            rhs >>= 1;
            let carry = (*self >> 7) & 1;
            *self <<= 1;
            *self ^= ((((carry & 1) as i8) << 7).wrapping_shr(7) as Self) & 0x1B;
        }

        *self = p.0;
    }
}

impl BitOr for GaloisField<u8> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<u8> for GaloisField<u8> {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Self::Output {
        Self(self.0 | rhs)
    }
}

impl BitOr<GaloisField<Self>> for u8 {
    type Output = Self;

    fn bitor(self, rhs: GaloisField<Self>) -> Self::Output {
        self | rhs.0
    }
}

impl BitOrAssign for GaloisField<u8> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u8> for GaloisField<u8> {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs;
    }
}

impl BitOrAssign<GaloisField<Self>> for u8 {
    fn bitor_assign(&mut self, rhs: GaloisField<Self>) {
        *self |= rhs.0;
    }
}

impl BitXor for GaloisField<u8> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXor<u8> for GaloisField<u8> {
    type Output = Self;

    fn bitxor(self, rhs: u8) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}

impl BitXor<GaloisField<Self>> for u8 {
    type Output = Self;

    fn bitxor(self, rhs: GaloisField<Self>) -> Self::Output {
        self ^ rhs.0
    }
}

impl BitXorAssign for GaloisField<u8> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<u8> for GaloisField<u8> {
    fn bitxor_assign(&mut self, rhs: u8) {
        self.0 ^= rhs;
    }
}

impl BitXorAssign<GaloisField<Self>> for u8 {
    fn bitxor_assign(&mut self, rhs: GaloisField<Self>) {
        *self ^= rhs.0;
    }
}

impl BitAnd for GaloisField<u8> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAnd<u8> for GaloisField<u8> {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self::Output {
        Self(self.0 & rhs)
    }
}

impl BitAnd<GaloisField<Self>> for u8 {
    type Output = Self;

    fn bitand(self, rhs: GaloisField<Self>) -> Self::Output {
        self & rhs.0
    }
}

impl BitAndAssign for GaloisField<u8> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<u8> for GaloisField<u8> {
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 &= rhs;
    }
}

impl BitAndAssign<GaloisField<Self>> for u8 {
    fn bitand_assign(&mut self, rhs: GaloisField<Self>) {
        *self &= rhs.0;
    }
}

impl Shr for GaloisField<u8> {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0 >> rhs.0)
    }
}

impl Shr<u8> for GaloisField<u8> {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shr<GaloisField<Self>> for u8 {
    type Output = Self;

    fn shr(self, rhs: GaloisField<Self>) -> Self::Output {
        self >> rhs.0
    }
}

impl ShrAssign for GaloisField<u8> {
    fn shr_assign(&mut self, rhs: Self) {
        self.0 >>= rhs.0;
    }
}

impl ShrAssign<u8> for GaloisField<u8> {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

impl ShrAssign<GaloisField<Self>> for u8 {
    fn shr_assign(&mut self, rhs: GaloisField<Self>) {
        *self >>= rhs.0;
    }
}

impl Shl for GaloisField<u8> {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << rhs.0)
    }
}

impl Shl<u8> for GaloisField<u8> {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shl<GaloisField<Self>> for u8 {
    type Output = Self;

    fn shl(self, rhs: GaloisField<Self>) -> Self::Output {
        self << rhs.0
    }
}

impl ShlAssign for GaloisField<u8> {
    fn shl_assign(&mut self, rhs: Self) {
        self.0 <<= rhs.0;
    }
}

impl ShlAssign<u8> for GaloisField<u8> {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs;
    }
}

impl ShlAssign<GaloisField<Self>> for u8 {
    fn shl_assign(&mut self, rhs: GaloisField<Self>) {
        *self <<= rhs.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            GaloisField::<u8>(5) + GaloisField::<u8>(7),
            GaloisField::<u8>(2)
        );
        assert_eq!(GaloisField::<u8>(5) + 7, GaloisField::<u8>(2));
        assert_eq!(7 + GaloisField::<u8>(5), 2);
    }

    #[test]
    fn sub() {
        assert_eq!(
            GaloisField::<u8>(5) - GaloisField::<u8>(7),
            GaloisField::<u8>(2)
        );
        assert_eq!(GaloisField::<u8>(5) - 7, GaloisField::<u8>(2));
        assert_eq!(7 - GaloisField::<u8>(5), 2);
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            GaloisField::<u8>(5) * GaloisField::<u8>(7),
            GaloisField::<u8>(27)
        );
        assert_eq!(GaloisField::<u8>(5) * 7, GaloisField::<u8>(27));
        assert_eq!(7 * GaloisField::<u8>(5), 27);
    }

    #[test]
    fn bitwise_or() {
        assert_eq!(
            GaloisField::<u8>(5) | GaloisField::<u8>(7),
            GaloisField::<u8>(7)
        );
        assert_eq!(GaloisField::<u8>(5) | 7, GaloisField::<u8>(7));
        assert_eq!(5 | GaloisField::<u8>(7), 7);
    }

    #[test]
    fn bitwise_xor() {
        assert_eq!(
            GaloisField::<u8>(5) ^ GaloisField::<u8>(7),
            GaloisField::<u8>(2)
        );
        assert_eq!(GaloisField::<u8>(5) ^ 7, GaloisField::<u8>(2));
        assert_eq!(5 ^ GaloisField::<u8>(7), 2);
    }

    #[test]
    fn bitwise_and() {
        assert_eq!(
            GaloisField::<u8>(5) & GaloisField::<u8>(7),
            GaloisField::<u8>(5)
        );
        assert_eq!(GaloisField::<u8>(5) & 7, GaloisField::<u8>(5));
        assert_eq!(5 & GaloisField::<u8>(7), 5);
    }

    #[test]
    fn bitwise_shr() {
        assert_eq!(
            GaloisField::<u8>(5) >> GaloisField::<u8>(7),
            GaloisField::<u8>(0)
        );
        assert_eq!(GaloisField::<u8>(5) >> 7, GaloisField::<u8>(0));
        assert_eq!(5 >> GaloisField::<u8>(7), 0);
    }

    #[test]
    fn bitwise_shl() {
        assert_eq!(
            GaloisField::<u8>(5) << GaloisField::<u8>(2),
            GaloisField::<u8>(20)
        );
        assert_eq!(GaloisField::<u8>(5) << 2, GaloisField::<u8>(20));
        assert_eq!(5 << GaloisField::<u8>(2), 20);
    }
}
