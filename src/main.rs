mod engine;
mod agents;
mod world;

use bevy::prelude::*;
use world::chunk::{WorldSeed, LoadedChunks, chunk_loading_system, setup_world};
use world::terrain::{TerrainGenerator, terrain_generation_system};
use engine::tick::agent_tick_system;
use agents::agent::spawn_agents;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(WorldSeed(12345))
        .insert_resource(TerrainGenerator::new(12345))
        .insert_resource(LoadedChunks::default())
        .add_systems(Startup, (setup_world, spawn_agents))
        .add_systems(Update, (
            chunk_loading_system,
            terrain_generation_system,
            world::chunk::debug_chunk_system,
            agent_tick_system,
        ))
        .run();
}
