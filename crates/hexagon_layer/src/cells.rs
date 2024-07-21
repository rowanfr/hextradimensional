use bevy::{math::{IVec2, Vec3}, prelude::{Changed, Component, Query}, transform::components::Transform};
mod iterators;
mod ops;

pub use iterators::*;

use crate::{HEX_SPACING, SQR_3, SQR_3_DIV_TWO};

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
pub struct HexId(IVec2);

impl HexId {
    pub const fn new(q: i32, r: i32) -> HexId {
        Self(IVec2 { x: q, y: r })
    }

    #[inline]
    pub fn q(&self) -> i32 {
        self.0.x
    }

    #[inline]
    pub fn r(&self) -> i32 {
        self.0.y
    }

    #[inline]
    pub fn s(&self) -> i32 {
        -self.q()-self.r()
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.q() as f32 * 1.5
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.q() as f32 * SQR_3_DIV_TWO + SQR_3 * self.r() as f32
    }

    #[inline]
    pub fn xyz(&self) -> Vec3 {
        Vec3 { x: self.x(), y: self.y(), z: 0. }
    }
}

pub(crate) fn update_transforms(
    mut hexagons: Query<(&mut Transform, &HexId), Changed<HexId>>,
) {
    for (mut pos, hex) in &mut hexagons {
        pos.translation.x = hex.x() * HEX_SPACING;
        pos.translation.y = hex.y() * HEX_SPACING;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum HexNeighbors {
    Direction1,
    Direction2,
    Direction3,
    Direction4,
    Direction5,
    Direction6,
}

impl HexNeighbors {
    pub const fn direction(&self) -> HexId {
        match self {
            HexNeighbors::Direction1 => HexId::new(0, -1),
            HexNeighbors::Direction2 => HexId::new(1, -1),
            HexNeighbors::Direction3 => HexId::new(1, 0),
            HexNeighbors::Direction4 => HexId::new(0, 1),
            HexNeighbors::Direction5 => HexId::new(-1, 1),
            HexNeighbors::Direction6 => HexId::new(-1, 0),
        }
    }

    pub const fn next(&self) -> HexNeighbors {
        match self {
            HexNeighbors::Direction1 => HexNeighbors::Direction2 ,
            HexNeighbors::Direction2 =>HexNeighbors::Direction3,
            HexNeighbors::Direction3 =>HexNeighbors::Direction4,
            HexNeighbors::Direction4 => HexNeighbors::Direction5,
            HexNeighbors::Direction5 => HexNeighbors::Direction6,
            HexNeighbors::Direction6 => HexNeighbors::Direction1,
        }
    }
}
