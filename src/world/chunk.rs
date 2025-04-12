use bevy::prelude::*;
use std::collections::HashMap;
use super::hex::HexCoord;
use super::terrain::TerrainGenerator;

pub const CHUNK_SIZE: i32 = 16; // 16x16 hex chunks

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn from_hex(hex: HexCoord) -> Self {
        Self {
            x: hex.q / CHUNK_SIZE,
            y: hex.r / CHUNK_SIZE,
        }
    }
    
    /// Calculate a hash value for this chunk coordinate
    /// Useful for seeding noise generators
    pub fn position_hash(&self) -> u32 {
        // Simple hash function for chunk coordinates
        // Use checked arithmetic to prevent overflow
        let x = self.x as u32;
        let y = self.y as u32;
        
        // Use wrapping_mul and wrapping_add to prevent overflow
        let x_hash = x.wrapping_mul(31);
        let y_hash = y.wrapping_mul(17);
        
        x_hash ^ y_hash
    }
}

/// Represents a tile in the world
#[derive(Debug, Clone, Component)]
pub struct Tile {
    pub elevation: f32,
    pub biome: Biome,
    // Add more properties as needed
}

/// Component for the position of a tile in the hex grid
#[derive(Debug, Clone, Component)]
#[allow(dead_code)]
pub struct HexPosition {
    pub coord: HexCoord,
    pub elevation: f32,
}

/// Represents different biome types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    Ocean,
    Plains,
    Forest,
    Mountains,
    #[allow(dead_code)]
    Desert,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            elevation: 0.0,
            biome: Biome::Plains,
        }
    }
}

#[derive(Component)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub tiles: HashMap<HexCoord, Tile>,
    pub is_loaded: bool,
    pub seed: u32,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            tiles: HashMap::new(),
            is_loaded: false,
            seed: coord.position_hash(),
        }
    }

    #[allow(dead_code)]
    pub fn load(&mut self) {
        if !self.is_loaded {
            self.is_loaded = true;
        }
    }

    #[allow(dead_code)]
    pub fn unload(&mut self) {
        self.tiles.clear();
        self.is_loaded = false;
    }
    
    /// Reset the chunk and regenerate its terrain
    /// Useful for world mutations, seasonal changes, or LLM-driven modifications
    #[allow(dead_code)]
    pub fn reset(&mut self, terrain_generator: &TerrainGenerator) {
        self.tiles.clear();
        terrain_generator.generate_chunk_terrain(self);
    }

    /// Serialize the chunk to bytes
    /// This is a placeholder implementation
    #[allow(dead_code)]
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In a real implementation, this would serialize the chunk data
        // For now, we'll just return an empty vector
        Ok(Vec::new())
    }
    
    /// Deserialize bytes into a chunk
    /// This is a placeholder implementation
    #[allow(dead_code)]
    pub fn deserialize(_bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        // In a real implementation, this would deserialize the bytes into a chunk
        // For now, we'll just return a new empty chunk
        Ok(Chunk::new(ChunkCoord::new(0, 0)))
    }
}

/// Resource to track the world seed
#[derive(Resource)]
pub struct WorldSeed(pub u32);

/// Resource to track loaded chunks
#[derive(Resource)]
pub struct LoadedChunks {
    pub chunks: HashMap<ChunkCoord, Entity>,
    pub load_radius: i32,
}

impl Default for LoadedChunks {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            load_radius: 2, // Default load radius
        }
    }
}

/// Component for entities that can trigger chunk loading
#[derive(Component)]
pub struct ChunkLoader {
    pub coord: ChunkCoord,
}

/// System to manage chunk loading based on player position
pub fn chunk_loading_system(
    mut commands: Commands,
    world_seed: Res<WorldSeed>,
    terrain_generator: Res<TerrainGenerator>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    chunk_loaders: Query<&ChunkLoader>,
    chunks: Query<(Entity, &Chunk)>,
) {
    // Get all chunk coordinates that need to be loaded
    let mut chunks_to_load = Vec::new();
    
    for loader in chunk_loaders.iter() {
        let loader_coord = loader.coord;
        
        // Check chunks within load radius
        for dx in -loaded_chunks.load_radius..=loaded_chunks.load_radius {
            for dy in -loaded_chunks.load_radius..=loaded_chunks.load_radius {
                let chunk_coord = ChunkCoord::new(
                    loader_coord.x + dx,
                    loader_coord.y + dy
                );
                
                if !loaded_chunks.chunks.contains_key(&chunk_coord) {
                    chunks_to_load.push(chunk_coord);
                }
            }
        }
    }
    
    // Load new chunks
    for chunk_coord in chunks_to_load {
        let mut chunk = Chunk::new(chunk_coord);
        chunk.seed = world_seed.0 ^ chunk_coord.position_hash();
        
        // Generate terrain using the terrain generator
        terrain_generator.generate_chunk_terrain(&mut chunk);
        chunk.is_loaded = true;
        
        let entity = commands.spawn((
            chunk,
            chunk_coord,
            Name::new(format!("Chunk({}, {})", chunk_coord.x, chunk_coord.y)),
        )).id();
        loaded_chunks.chunks.insert(chunk_coord, entity);
    }
    
    // Unload chunks that are too far from any loader
    let mut chunks_to_unload = Vec::new();
    
    for (entity, chunk) in chunks.iter() {
        let mut should_unload = true;
        
        for loader in chunk_loaders.iter() {
            let dx = (chunk.coord.x - loader.coord.x).abs();
            let dy = (chunk.coord.y - loader.coord.y).abs();
            
            if dx <= loaded_chunks.load_radius && dy <= loaded_chunks.load_radius {
                should_unload = false;
                break;
            }
        }
        
        if should_unload {
            chunks_to_unload.push((entity, chunk.coord));
        }
    }
    
    // Unload chunks
    for (entity, coord) in chunks_to_unload {
        commands.entity(entity).despawn();
        loaded_chunks.chunks.remove(&coord);
    }
}

/// System to update chunk loaders based on player movement
#[allow(dead_code)]
pub fn update_chunk_loaders_system(
    _commands: Commands,
    _chunk_loaders: Query<&mut ChunkLoader>,
    // Add player position query here
) {
    // This would update ChunkLoader components based on player position
    // For now, this is a placeholder
}

/// Setup function to initialize the world
pub fn setup_world(mut commands: Commands) {
    // Spawn a test entity with a ChunkLoader component
    commands.spawn((
        ChunkLoader { coord: ChunkCoord::new(0, 0) },
        Name::new("TestLoader"),
    ));
    
    info!("World setup complete. Test loader spawned at (0, 0).");
}

/// Debug system to log information about loaded chunks
pub fn debug_chunk_system(
    chunks: Query<&Chunk>,
    loaded_chunks: Res<LoadedChunks>,
) {
    info!("Loaded chunks: {}", loaded_chunks.chunks.len());
    for chunk in chunks.iter() {
        info!("Chunk({}, {}): {} tiles", 
            chunk.coord.x, 
            chunk.coord.y, 
            chunk.tiles.len()
        );
    }
}

/// System to spawn tile entities for rendering
/// This can be used when you're ready to implement rendering in Milestone 2.3
#[allow(dead_code)]
pub fn spawn_tile_entities_system(
    _commands: Commands,
    _chunks: Query<&Chunk>,
) {
    // This is a placeholder for future implementation
    // When you're ready to implement rendering, you can use this system
    // to spawn tile entities for each tile in each chunk
    
    // Example implementation:
    /*
    for chunk in chunks.iter() {
        for (hex, tile) in chunk.tiles.iter() {
            commands.spawn((
                SpriteBundle {
                    // Sprite configuration
                },
                HexPosition {
                    coord: *hex,
                    elevation: tile.elevation,
                },
                tile.clone(),
            ));
        }
    }
    */
}

impl LoadedChunks {
    /// Get neighboring chunks for a given chunk coordinate
    #[allow(dead_code)]
    pub fn get_neighbor_chunks(&self, coord: ChunkCoord) -> Vec<Entity> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the center chunk
                }
                
                let neighbor_coord = ChunkCoord::new(coord.x + dx, coord.y + dy);
                if let Some(&entity) = self.chunks.get(&neighbor_coord) {
                    neighbors.push(entity);
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_coord_from_hex() {
        // Test basic conversion
        let hex = HexCoord::new(10, 5);
        let chunk = ChunkCoord::from_hex(hex);
        assert_eq!(chunk.x, 0);
        assert_eq!(chunk.y, 0);
        
        // Test edge cases
        let hex = HexCoord::new(16, 16);
        let chunk = ChunkCoord::from_hex(hex);
        assert_eq!(chunk.x, 1);
        assert_eq!(chunk.y, 1);
        
        // Test negative coordinates
        let hex = HexCoord::new(-16, -16);
        let chunk = ChunkCoord::from_hex(hex);
        assert_eq!(chunk.x, -1);
        assert_eq!(chunk.y, -1);
        
        // Test boundary cases
        let hex = HexCoord::new(15, 15);
        let chunk = ChunkCoord::from_hex(hex);
        assert_eq!(chunk.x, 0);
        assert_eq!(chunk.y, 0);
        
        let hex = HexCoord::new(16, 15);
        let chunk = ChunkCoord::from_hex(hex);
        assert_eq!(chunk.x, 1);
        assert_eq!(chunk.y, 0);
    }
} 