use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use super::hex::HexCoord;
use super::chunk::{Chunk, Biome, CHUNK_SIZE};

#[derive(Resource)]
pub struct TerrainGenerator {
    noise: Perlin,
    scale: f64,
    octaves: i32,
    persistence: f64,
    // Biome thresholds
    ocean_threshold: f32,
    plains_threshold: f32,
    forest_threshold: f32,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Perlin::new(seed),
            scale: 0.01,
            octaves: 4,
            persistence: 0.5,
            ocean_threshold: 0.2,
            plains_threshold: 0.4,
            forest_threshold: 0.7,
        }
    }
    
    /// Create a terrain generator with custom parameters
    pub fn with_params(
        seed: u32,
        scale: f64,
        octaves: i32,
        persistence: f64,
        ocean_threshold: f32,
        plains_threshold: f32,
        forest_threshold: f32,
    ) -> Self {
        Self {
            noise: Perlin::new(seed),
            scale,
            octaves,
            persistence,
            ocean_threshold,
            plains_threshold,
            forest_threshold,
        }
    }

    pub fn generate_elevation(&self, coord: &HexCoord) -> f32 {
        let mut elevation = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = self.scale;
        let mut max_value = 0.0;

        for _ in 0..self.octaves {
            let x = coord.q as f64 * frequency;
            let y = coord.r as f64 * frequency;
            
            elevation += amplitude * self.noise.get([x, y]) as f64;
            max_value += amplitude;
            
            amplitude *= self.persistence;
            frequency *= 2.0;
        }

        // Normalize to 0-1 range
        elevation = (elevation / max_value + 1.0) / 2.0;
        
        elevation as f32
    }
    
    pub fn get_biome(&self, elevation: f32) -> Biome {
        if elevation < self.ocean_threshold {
            Biome::Ocean
        } else if elevation < self.plains_threshold {
            Biome::Plains
        } else if elevation < self.forest_threshold {
            Biome::Forest
        } else {
            Biome::Mountains
        }
    }
    
    /// Generate terrain for a specific chunk
    pub fn generate_chunk_terrain(&self, chunk: &mut Chunk) {
        for q in 0..CHUNK_SIZE {
            for r in 0..CHUNK_SIZE {
                // Calculate the hex coordinates for this tile within the chunk
                // Use checked arithmetic to prevent overflow
                let hex_q = match chunk.coord.x.checked_mul(CHUNK_SIZE) {
                    Some(q_base) => match q_base.checked_add(q) {
                        Some(q_val) => q_val,
                        None => continue, // Skip this tile if there's an overflow
                    },
                    None => continue, // Skip this tile if there's an overflow
                };
                
                let hex_r = match chunk.coord.y.checked_mul(CHUNK_SIZE) {
                    Some(r_base) => match r_base.checked_add(r) {
                        Some(r_val) => r_val,
                        None => continue, // Skip this tile if there's an overflow
                    },
                    None => continue, // Skip this tile if there's an overflow
                };
                
                let hex = HexCoord::new(hex_q, hex_r);
                
                let elevation = self.generate_elevation(&hex);
                let biome = self.get_biome(elevation);
                
                chunk.tiles.insert(hex, super::chunk::Tile { elevation, biome });
            }
        }
    }
}

// System to generate terrain for chunks
pub fn terrain_generation_system(
    mut chunks: Query<&mut Chunk>,
    generator: Res<TerrainGenerator>,
) {
    for mut chunk in chunks.iter_mut() {
        if !chunk.is_loaded {
            continue;
        }

        for (hex, tile) in chunk.tiles.iter_mut() {
            let elevation = generator.generate_elevation(hex);
            tile.elevation = elevation;
            tile.biome = generator.get_biome(elevation);
        }
    }
} 