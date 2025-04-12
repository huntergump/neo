use std::ops::{Add, Sub};
use std::fmt;
use bevy::prelude::*;

/// HexDirection represents the six possible directions in a hex grid
/// 
/// The directions are defined in a pointy-top orientation:
/// 
///     NW  N
///   W  •  E
///     SW  S
/// 
/// Where:
/// - E  = East (right)
/// - NE = Northeast (top-right)
/// - NW = Northwest (top-left)
/// - W  = West (left)
/// - SW = Southwest (bottom-left)
/// - SE = Southeast (bottom-right)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexDirection {
    E,  // East (right)
    NE, // Northeast (top-right)
    NW, // Northwest (top-left)
    W,  // West (left)
    SW, // Southwest (bottom-left)
    SE, // Southeast (bottom-right)
}

impl HexDirection {
    /// Convert a direction to a hex coordinate offset
    pub fn to_offset(&self) -> HexCoord {
        match self {
            HexDirection::E  => HexCoord::new(1, 0),
            HexDirection::NE => HexCoord::new(1, -1),
            HexDirection::NW => HexCoord::new(0, -1),
            HexDirection::W  => HexCoord::new(-1, 0),
            HexDirection::SW => HexCoord::new(-1, 1),
            HexDirection::SE => HexCoord::new(0, 1),
        }
    }

    /// Get all possible directions
    pub fn all() -> [HexDirection; 6] {
        [
            HexDirection::E,
            HexDirection::NE,
            HexDirection::NW,
            HexDirection::W,
            HexDirection::SW,
            HexDirection::SE,
        ]
    }
    
    /// Rotate this direction 60 degrees counterclockwise
    pub fn rotate_left(&self) -> HexDirection {
        match self {
            HexDirection::E  => HexDirection::NE,
            HexDirection::NE => HexDirection::NW,
            HexDirection::NW => HexDirection::W,
            HexDirection::W  => HexDirection::SW,
            HexDirection::SW => HexDirection::SE,
            HexDirection::SE => HexDirection::E,
        }
    }
    
    /// Rotate this direction 60 degrees clockwise
    pub fn rotate_right(&self) -> HexDirection {
        match self {
            HexDirection::E  => HexDirection::SE,
            HexDirection::NE => HexDirection::E,
            HexDirection::NW => HexDirection::NE,
            HexDirection::W  => HexDirection::NW,
            HexDirection::SW => HexDirection::W,
            HexDirection::SE => HexDirection::SW,
        }
    }
    
    /// Get the opposite direction
    pub fn opposite(&self) -> HexDirection {
        match self {
            HexDirection::E  => HexDirection::W,
            HexDirection::NE => HexDirection::SW,
            HexDirection::NW => HexDirection::SE,
            HexDirection::W  => HexDirection::E,
            HexDirection::SW => HexDirection::NE,
            HexDirection::SE => HexDirection::NW,
        }
    }
}

/// HexCoord represents a position in a hex grid using axial coordinates (q, r)
/// 
/// In axial coordinates:
/// - q represents the column (increasing to the right)
/// - r represents the row (increasing to the bottom-right)
/// 
/// The coordinate system uses a pointy-top orientation:
/// 
///     NW  N
///   W  •  E
///     SW  S
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32, // Column (increasing to the right)
    pub r: i32, // Row (increasing to the bottom-right)
}

impl HexCoord {
    /// Create a new hex coordinate
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Convert to pixel coordinates (for rendering)
    /// 
    /// This assumes a pointy-top orientation with the origin at the center
    pub fn to_pixel(&self, size: f32) -> Vec2 {
        let x = size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = size * (3.0 / 2.0 * self.r as f32);
        Vec2::new(x, y)
    }

    /// Get neighboring hex coordinates in all six directions
    pub fn neighbors(&self) -> Vec<HexCoord> {
        HexDirection::all()
            .iter()
            .map(|&dir| *self + dir.to_offset())
            .collect()
    }

    /// Get a neighbor in a specific direction
    pub fn neighbor(&self, direction: HexDirection) -> HexCoord {
        *self + direction.to_offset()
    }

    /// Calculate distance to another hex
    pub fn distance_to(&self, other: &HexCoord) -> i32 {
        let dx = (self.q - other.q).abs();
        let dy = (self.r - other.r).abs();
        let dz = (-self.q - self.r + other.q + other.r).abs();
        
        (dx + dy + dz) / 2
    }
    
    /// Get the direction from this hex to another hex
    /// 
    /// Returns None if the other hex is not directly adjacent
    pub fn direction_to(&self, other: &HexCoord) -> Option<HexDirection> {
        let delta = *other - *self;
        for dir in HexDirection::all() {
            if dir.to_offset() == delta {
                return Some(dir);
            }
        }
        None
    }

    /// Get the third coordinate (s) in cube coordinates
    /// 
    /// In cube coordinates, q + r + s = 0
    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    /// Create from cube coordinates (q, r, s)
    pub fn from_cube(q: i32, r: i32, s: i32) -> Option<Self> {
        if q + r + s == 0 {
            Some(Self::new(q, r))
        } else {
            None
        }
    }
}

impl fmt::Display for HexCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hex({}, {})", self.q, self.r)
    }
}

impl Add for HexCoord {
    type Output = HexCoord;

    fn add(self, other: HexCoord) -> HexCoord {
        HexCoord::new(self.q + other.q, self.r + other.r)
    }
}

impl Sub for HexCoord {
    type Output = HexCoord;

    fn sub(self, other: HexCoord) -> HexCoord {
        HexCoord::new(self.q - other.q, self.r - other.r)
    }
}

/// Component for entities that exist in the hex grid
#[derive(Component)]
pub struct HexPosition {
    pub coord: HexCoord,
    pub elevation: f32,
} 