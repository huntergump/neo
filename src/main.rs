mod engine;
mod agents;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, agents::agent::spawn_agents)
        .add_systems(Update, engine::tick::agent_tick_system)
        .run();
}
