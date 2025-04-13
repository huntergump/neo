use bevy::prelude::*;
use rand::random;

// Constants for weather calculations
const SECONDS_PER_DAY: f32 = 24.0 * 60.0 * 60.0;
const TAU: f32 = std::f32::consts::PI * 2.0;

// Weather event for notifying systems of significant weather changes
#[derive(Event, Debug)]
pub struct WeatherChanged {
    pub temperature: f32,
    pub humidity: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub precipitation: f32,
    pub cloud_cover: f32,
}

/// System for managing weather in the simulation
#[derive(Debug, Clone, Component)]
pub struct WeatherSystem {
    pub temperature: f32,     // Celsius
    pub humidity: f32,        // 0.0 to 1.0
    pub wind_speed: f32,      // m/s
    pub wind_direction: f32,  // radians
    pub precipitation: f32,   // mm/hour
    pub cloud_cover: f32,     // 0.0 to 1.0
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            temperature: 20.0, // Celsius
            humidity: 0.5,     // 0.0 to 1.0
            wind_speed: 0.0,   // m/s
            wind_direction: 0.0, // radians
            precipitation: 0.0,  // mm/hour
            cloud_cover: 0.3,   // 0.0 to 1.0
        }
    }
}

/// Plugin for the weather system
pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WeatherChanged>()
            .add_systems(Startup, spawn_weather_system)
            .add_systems(Update, (
                update_weather_system, 
                process_weather_changes,
                clear_weather_events,
            ));
    }
}

/// Spawns the initial weather system
fn spawn_weather_system(mut commands: Commands) {
    commands.spawn(WeatherSystem::default());
}

/// System for updating weather
pub fn update_weather_system(
    time: Res<Time>,
    mut query: Query<&mut WeatherSystem>,
    mut events: EventWriter<WeatherChanged>,
) {
    let delta = time.delta_secs();
    
    for mut weather in query.iter_mut() {
        // Store old values to detect significant changes
        let old_temp = weather.temperature;
        let old_humidity = weather.humidity;
        let old_wind_speed = weather.wind_speed;
        let old_precipitation = weather.precipitation;
        let old_cloud_cover = weather.cloud_cover;
        
        // Simple weather simulation
        // Temperature varies with time of day
        let time_of_day = (time.elapsed_secs() % SECONDS_PER_DAY) / SECONDS_PER_DAY;
        let base_temp = 15.0 + 10.0 * (time_of_day * TAU).sin();
        
        // Add some noise to temperature
        weather.temperature = base_temp + (random::<f32>() - 0.5) * 2.0;
        
        // Wind changes slowly
        weather.wind_speed += (random::<f32>() - 0.5) * delta * 0.1;
        weather.wind_speed = weather.wind_speed.clamp(0.0, 20.0);
        
        weather.wind_direction += (random::<f32>() - 0.5) * delta * 0.1;
        if weather.wind_direction > TAU {
            weather.wind_direction -= TAU;
        }
        
        // Humidity and precipitation
        weather.humidity += (random::<f32>() - 0.5) * delta * 0.01;
        weather.humidity = weather.humidity.clamp(0.0, 1.0);
        
        // Cloud cover is influenced by humidity and wind
        weather.cloud_cover += (weather.humidity - 0.5) * delta * 0.01;
        weather.cloud_cover = weather.cloud_cover.clamp(0.0, 1.0);
        
        // More likely to rain when humidity is high and cloud cover is significant
        if weather.humidity > 0.8 && weather.cloud_cover > 0.6 && random::<f32>() < 0.1 {
            weather.precipitation = random::<f32>() * 10.0;
        } else {
            weather.precipitation *= 0.95; // Gradually decrease precipitation
        }
        
        // Check for significant changes to emit events
        let temp_change = (weather.temperature - old_temp).abs();
        let humidity_change = (weather.humidity - old_humidity).abs();
        let wind_change = (weather.wind_speed - old_wind_speed).abs();
        let precip_change = (weather.precipitation - old_precipitation).abs();
        let cloud_change = (weather.cloud_cover - old_cloud_cover).abs();
        
        // Emit event if any significant change occurred
        if temp_change > 2.0 || humidity_change > 0.1 || wind_change > 1.0 || 
           precip_change > 1.0 || cloud_change > 0.1 {
            events.send(WeatherChanged {
                temperature: weather.temperature,
                humidity: weather.humidity,
                wind_speed: weather.wind_speed,
                wind_direction: weather.wind_direction,
                precipitation: weather.precipitation,
                cloud_cover: weather.cloud_cover,
            });
        }
    }
}

/// System that processes weather changes and updates UI or other systems
pub fn process_weather_changes(
    mut events: EventReader<WeatherChanged>,
) {
    for event in events.read() {
        // This is a placeholder for future UI updates, logging, or other systems
        // that need to react to weather changes
        info!(
            "Weather changed: Temp={:.1}°C, Humidity={:.2}, Wind={:.1}m/s, Rain={:.1}mm/hr, Clouds={:.2}",
            event.temperature, event.humidity, event.wind_speed, event.precipitation, event.cloud_cover
        );
    }
}

/// System that clears old weather events to prevent memory leaks
pub fn clear_weather_events(
    mut events: ResMut<Events<WeatherChanged>>,
) {
    // Clear all events after they've been processed
    events.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_weather_system_default() {
        let weather = WeatherSystem::default();
        
        assert_eq!(weather.temperature, 20.0);
        assert_eq!(weather.humidity, 0.5);
        assert_eq!(weather.wind_speed, 0.0);
        assert_eq!(weather.wind_direction, 0.0);
        assert_eq!(weather.precipitation, 0.0);
        assert_eq!(weather.cloud_cover, 0.3);
    }
    
    #[test]
    fn test_weather_clamping() {
        let mut app = App::new();
        app.add_plugins(WeatherPlugin);
        
        // Run the system for a few frames to ensure values stay within bounds
        for _ in 0..10 {
            app.update();
            
            let weather = app.world.query::<&WeatherSystem>().single();
            
            // Check that values are properly clamped
            assert!(weather.humidity >= 0.0 && weather.humidity <= 1.0, 
                "Humidity should be clamped between 0.0 and 1.0, got {}", weather.humidity);
            
            assert!(weather.wind_speed >= 0.0 && weather.wind_speed <= 20.0, 
                "Wind speed should be clamped between 0.0 and 20.0, got {}", weather.wind_speed);
            
            assert!(weather.wind_direction >= 0.0 && weather.wind_direction < TAU, 
                "Wind direction should be clamped between 0.0 and TAU, got {}", weather.wind_direction);
            
            assert!(weather.cloud_cover >= 0.0 && weather.cloud_cover <= 1.0, 
                "Cloud cover should be clamped between 0.0 and 1.0, got {}", weather.cloud_cover);
            
            assert!(weather.precipitation >= 0.0, 
                "Precipitation should never be negative, got {}", weather.precipitation);
        }
    }
    
    #[test]
    fn test_weather_events() {
        let mut app = App::new();
        app.add_plugins(WeatherPlugin);
        
        // Run the system for a few frames to generate some events
        for _ in 0..10 {
            app.update();
        }
        
        // Check that events were generated
        let mut events = app.world.resource_mut::<Events<WeatherChanged>>();
        let mut reader = events.get_reader();
        let events: Vec<&WeatherChanged> = reader.read(&events).collect();
        
        // We should have at least one weather change event
        assert!(!events.is_empty(), "No weather change events were generated");
    }
} 