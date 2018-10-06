use std::ops::{Add, Shl, ShlAssign};
use std::fmt::{Debug, Display, Formatter, Result};
use std::fmt::Binary;

custom_derive! {
    #[derive(NewtypeNot, NewtypeBitAnd, NewtypeBitOr, NewtypeBitXor)]
    #[derive(Clone, Copy)]
    pub struct WrappingRotating(pub u32);
}

#[derive(Clone)]
pub struct DigestBuffer {
    pub a: WrappingRotating,
    pub b: WrappingRotating,
    pub c: WrappingRotating,
    pub d: WrappingRotating,
}

// Let us use + syntax for component-wise addition as per the spec.
impl Add for DigestBuffer {
    type Output = DigestBuffer;

    fn add(self, rhs: DigestBuffer) -> DigestBuffer {
        DigestBuffer {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
        }
    }
}

// Let us use a + b syntax for wrapping add as per the spec.
impl Add for WrappingRotating {
    type Output = WrappingRotating;

    fn add(self, rhs: WrappingRotating) -> WrappingRotating {
        WrappingRotating(self.0.wrapping_add(rhs.0))
    }
}
impl Add<u32> for WrappingRotating {
    type Output = WrappingRotating;

    fn add(self, rhs: u32) -> WrappingRotating {
        WrappingRotating(self.0.wrapping_add(rhs))
    }
}

// Let us use << syntax for rotate left as per the spec.
impl Shl<u32> for WrappingRotating {
    type Output = WrappingRotating;

    fn shl(self, rhs: u32) -> WrappingRotating {
        WrappingRotating(self.0.rotate_left(rhs))
    }
}
impl ShlAssign<u32> for WrappingRotating {
    fn shl_assign(&mut self, rhs: u32) { self.0 = self.0.rotate_left(rhs) }
}

// Debug information.
impl Display for WrappingRotating {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "WR({})", self.0)
    }
}
impl Debug for WrappingRotating {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}, {}, {}, {}", (self.0 >> 24) & 0xff, (self.0 >> 16) & 0xff, (self.0 >> 8) & 0xff, self.0 & 0xff)
    }
}
impl Binary for WrappingRotating {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({:>032b})", self.0)
    }
}
