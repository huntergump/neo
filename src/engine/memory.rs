use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Resource for tracking memory usage
#[derive(Resource, Debug)]
pub struct MemoryProfiler {
    /// Last time memory was profiled
    pub last_profile_time: Instant,
    /// Interval between memory profiles in seconds
    pub profile_interval: f32,
    /// Memory usage history
    pub memory_history: Vec<(f32, usize)>,
    /// Component counts by type
    pub component_counts: HashMap<String, usize>,
    /// Resource sizes by type
    pub resource_sizes: HashMap<String, usize>,
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self {
            last_profile_time: Instant::now(),
            profile_interval: 5.0, // Profile every 5 seconds
            memory_history: Vec::new(),
            component_counts: HashMap::new(),
            resource_sizes: HashMap::new(),
        }
    }
}

/// Resource to request memory profiling
#[derive(Resource, Default)]
pub struct MemoryProfilingRequest {
    pub timestamp: f32,
}

/// System for profiling memory usage
pub fn memory_profiling_system(
    mut profiler: ResMut<MemoryProfiler>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let current_time = time.elapsed_secs();
    
    // Check if it's time to profile again
    if profiler.last_profile_time.elapsed() < Duration::from_secs_f32(profiler.profile_interval) {
        return;
    }
    
    // Update last profile time
    profiler.last_profile_time = Instant::now();
    
    // Log memory usage
    info!("Memory profiling at time: {:.2}", current_time);
    
    // We'll use a separate system to gather memory statistics
    commands.insert_resource(MemoryProfilingRequest {
        timestamp: current_time,
    });
}

/// System to gather memory statistics
pub fn gather_memory_stats(
    request: Res<MemoryProfilingRequest>,
    mut profiler: ResMut<MemoryProfiler>,
    query: Query<(), ()>,
) {
    // Count entities
    let entity_count = query.iter().count();
    
    // Record memory usage (approximate)
    let total_memory = entity_count * 64; // Rough estimate: 64 bytes per entity
    
    // Record memory usage - convert to usize to match the expected type
    profiler.memory_history.push((request.timestamp, total_memory as usize));
    
    // Keep only the last 100 memory readings
    if profiler.memory_history.len() > 100 {
        profiler.memory_history.remove(0);
    }
    
    // Log memory usage
    info!("Memory usage: {} bytes ({} entities)", total_memory, entity_count);
    
    // Calculate memory growth rate
    if profiler.memory_history.len() >= 2 {
        let (time1, mem1) = profiler.memory_history[profiler.memory_history.len() - 2];
        let (time2, mem2) = profiler.memory_history[profiler.memory_history.len() - 1];
        let time_diff = time2 - time1;
        let mem_diff = mem2 as i64 - mem1 as i64;
        
        if time_diff > 0.0 {
            let growth_rate = mem_diff as f64 / time_diff as f64;
            info!("Memory growth rate: {:.2} bytes/second", growth_rate);
            
            // Alert if memory is growing too fast
            if growth_rate > 1024.0 * 1024.0 { // More than 1MB per second
                warn!("High memory growth rate detected: {:.2} bytes/second", growth_rate);
            }
        }
    }
}

/// Plugin for memory profiling
pub struct MemoryProfilingPlugin;

impl Plugin for MemoryProfilingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MemoryProfiler>()
            .init_resource::<MemoryProfilingRequest>()
            .add_systems(Update, memory_profiling_system)
            .add_systems(Update, gather_memory_stats.after(memory_profiling_system));
    }
} 