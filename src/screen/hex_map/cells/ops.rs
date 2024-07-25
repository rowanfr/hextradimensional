use super::{HexDirection, HexId};
use core::ops::{Add, AddAssign};

impl Add<HexDirection> for HexId {
    type Output = HexId;
    fn add(self, rhs: HexDirection) -> Self::Output {
        self + rhs.direction()
    }
}

impl Add for HexId {
    type Output = HexId;
    fn add(self, rhs: Self) -> Self::Output {
        HexId::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl AddAssign for HexId {
    fn add_assign(&mut self, rhs: Self) {
        self.0.y += rhs.0.y;
        self.0.x += rhs.0.x;
    }
}

impl AddAssign<HexDirection> for HexId {
    fn add_assign(&mut self, rhs: HexDirection) {
        *self += rhs.direction();
    }
}
