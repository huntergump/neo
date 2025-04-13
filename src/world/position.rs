use bevy::prelude::*;

/// Represents a position in the world
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    pub fn from_vec2(vec: Vec2) -> Self {
        Self { x: vec.x, y: vec.y }
    }

    /// Calculate the distance to another position
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Linear interpolation between two positions
    pub fn lerp(&self, to: &Position, t: f32) -> Position {
        Position {
            x: self.x + (to.x - self.x) * t,
            y: self.y + (to.y - self.y) * t,
        }
    }

    /// Get the direction vector to another position
    pub fn direction_to(&self, other: &Position) -> Vec2 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let length = (dx * dx + dy * dy).sqrt();
        if length > 0.0 {
            Vec2::new(dx / length, dy / length)
        } else {
            Vec2::ZERO
        }
    }

    /// Move towards another position by a given distance
    pub fn move_towards(&self, target: &Position, distance: f32) -> Position {
        let direction = self.direction_to(target);
        Position {
            x: self.x + direction.x * distance,
            y: self.y + direction.y * distance,
        }
    }
}
