use bevy::prelude::*;

/// Represents a hexagonal grid coordinate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn to_world_position(&self, size: f32) -> Vec2 {
        let x = size * (3.0f32.sqrt() * self.q as f32 + 3.0f32.sqrt() / 2.0 * self.r as f32);
        let y = size * (3.0 / 2.0 * self.r as f32);
        Vec2::new(x, y)
    }

    pub fn from_world_position(pos: Vec2, size: f32) -> Self {
        let q = (2.0/3.0 * pos.x / size) as i32;
        let r = (-1.0/3.0 * pos.x + 3.0f32.sqrt()/3.0 * pos.y / size) as i32;
        Self::new(q, r)
    }
}

/// Component for entities that exist on a hexagonal grid
#[derive(Component, Debug, Clone, Copy)]
pub struct HexPosition {
    pub coord: HexCoord,
    pub size: f32,
}

impl HexPosition {
    pub fn new(coord: HexCoord, size: f32) -> Self {
        Self { coord, size }
    }

    pub fn to_world_position(&self) -> Vec2 {
        self.coord.to_world_position(self.size)
    }
}

/// System for updating hex positions
pub fn update_hex_positions(
    mut query: Query<(&HexPosition, &mut Transform)>,
) {
    for (hex_pos, mut transform) in query.iter_mut() {
        transform.translation = hex_pos.to_world_position().extend(0.0);
    }
} 