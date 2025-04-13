use bevy::prelude::*;

/// System for managing simulation time
#[derive(Debug, Clone, Component)]
pub struct TimeSystem {
    pub current_time: f32,
    pub day_length: f32,
    pub time_scale: f32,
}

impl Default for TimeSystem {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            day_length: 24.0 * 60.0, // 24 hours in minutes
            time_scale: 1.0,
        }
    }
}

/// System for updating the time
pub fn update_time_system(
    time: Res<Time>,
    mut query: Query<&mut TimeSystem>,
) {
    let delta = time.delta_secs();
    
    for mut time_system in query.iter_mut() {
        time_system.current_time += delta * time_system.time_scale;
        
        // Wrap around at day length
        if time_system.current_time >= time_system.day_length {
            time_system.current_time -= time_system.day_length;
        }
    }
} 