mod engine;
mod agents;
mod world;

use bevy::prelude::*;
use world::chunk::{WorldSeed, LoadedChunks, chunk_loading_system, setup_world, ChunkLoaded, ChunkUnloaded};
use world::terrain::{TerrainGenerator, terrain_generation_system};
use engine::tick::{agent_tick_system, AgentTickCompleted};
use agents::agent::spawn_agents;
use std::collections::HashMap;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::WindowMode;
use bevy::window::WindowResolution;
use engine::{update_time_system, WeatherPlugin};

/// System sets for organizing simulation systems
/// 
/// This enum defines the logical groups of systems in our simulation:
/// - WorldGeneration: Handles terrain and chunk generation
/// - AgentProcessing: Manages agent behavior and interactions
/// - Debug: Provides debugging information and visualization
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SimulationSet {
    WorldGeneration,
    AgentProcessing,
    Debug,
}

/// Configuration for the simulation
#[derive(Resource)]
pub struct SimulationConfig {
    /// World seed for terrain generation
    pub world_seed: u32,
    /// Chunk load radius
    pub chunk_load_radius: i32,
    /// Simulation speed
    pub simulation_speed: f64,
    /// Number of agents to spawn
    pub agent_count: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            world_seed: 42,
            chunk_load_radius: 5,
            simulation_speed: 60.0,
            agent_count: 100,
        }
    }
}

fn main() {
    // Create a single instance of the config to reuse
    let config = SimulationConfig::default();
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Neo Simulation".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(WeatherPlugin)
        .add_event::<ChunkLoaded>()
        .add_event::<ChunkUnloaded>()
        .add_event::<AgentTickCompleted>()
        .insert_resource(WorldSeed(config.world_seed))
        .insert_resource(TerrainGenerator::default())
        .insert_resource(LoadedChunks {
            chunks: HashMap::new(),
            load_radius: config.chunk_load_radius,
            tile_entity_map: HashMap::new(),
        })
        .insert_resource(Time::<Fixed>::from_hz(config.simulation_speed))
        .insert_resource(Time::<Virtual>::default())
        .insert_resource(config)
        .add_systems(Startup, (setup_world, spawn_agents))
        .add_systems(Update, (
            chunk_loading_system,
            terrain_generation_system,
        ).in_set(SimulationSet::WorldGeneration))
        .add_systems(Update, (
            agent_tick_system,
            update_time_system,
        ).in_set(SimulationSet::AgentProcessing))
        .add_systems(Update, (
            world::chunk::debug_chunk_system,
        ).in_set(SimulationSet::Debug))
        .configure_sets(Update, (
            SimulationSet::WorldGeneration,
            SimulationSet::AgentProcessing,
            SimulationSet::Debug,
        ).chain())
        .run();
}
