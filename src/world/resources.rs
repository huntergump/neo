use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCategory {
    Basic,      // Food, Water, Oxygen
    Energy,     // Energy, Fuel
    Material,   // Metal, Wood, Stone
    Special,    // Rare resources, artifacts
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceMetadata {
    pub category: ResourceCategory,
    pub is_renewable: bool,
    pub decay_rate: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Food,
    Water,
    Energy,
    Metal,
    Oxygen,
}

impl ResourceType {
    pub fn metadata(&self) -> ResourceMetadata {
        match self {
            ResourceType::Food => ResourceMetadata {
                category: ResourceCategory::Basic,
                is_renewable: true,
                decay_rate: 0.1,
            },
            ResourceType::Water => ResourceMetadata {
                category: ResourceCategory::Basic,
                is_renewable: true,
                decay_rate: 0.0,
            },
            ResourceType::Energy => ResourceMetadata {
                category: ResourceCategory::Energy,
                is_renewable: true,
                decay_rate: 0.2,
            },
            ResourceType::Metal => ResourceMetadata {
                category: ResourceCategory::Material,
                is_renewable: false,
                decay_rate: 0.0,
            },
            ResourceType::Oxygen => ResourceMetadata {
                category: ResourceCategory::Basic,
                is_renewable: true,
                decay_rate: 0.0,
            },
        }
    }
}

#[derive(Event, Debug)]
pub struct ResourceChanged {
    pub resource_type: ResourceType,
    pub old_amount: f32,
    pub new_amount: f32,
    pub source: Option<Entity>,
}

#[derive(Debug, Clone, Resource)]
pub struct ResourceManager {
    pub resources: HashMap<ResourceType, f32>,
    pub max_capacity: HashMap<ResourceType, f32>,
    pub regeneration_rate: f32,
    pub depletion_rate: f32,
}

impl ResourceManager {
    pub fn new() -> Self {
        let mut max_capacity = HashMap::new();
        max_capacity.insert(ResourceType::Food, 1000.0);
        max_capacity.insert(ResourceType::Water, 1000.0);
        max_capacity.insert(ResourceType::Energy, 1000.0);
        max_capacity.insert(ResourceType::Metal, 1000.0);
        max_capacity.insert(ResourceType::Oxygen, 1000.0);
        
        Self {
            resources: HashMap::new(),
            max_capacity,
            regeneration_rate: 1.0,
            depletion_rate: 1.0,
        }
    }

    pub fn add(&mut self, resource: ResourceType, amount: f32, event_writer: Option<&mut EventWriter<ResourceChanged>>) -> f32 {
        let max = *self.max_capacity.get(&resource).unwrap_or(&f32::INFINITY);
        let current = *self.resources.entry(resource).or_insert(0.0);
        let new_amount = (current + amount).min(max);
        let added = new_amount - current;
        
        if added > 0.0 {
            let old = current;
            *self.resources.entry(resource).or_insert(0.0) = new_amount;
            
            if let Some(events) = event_writer {
                events.send(ResourceChanged {
                    resource_type: resource,
                    old_amount: old,
                    new_amount,
                    source: None,
                });
            }
        }
        
        added
    }

    pub fn consume(&mut self, resource: ResourceType, amount: f32, event_writer: Option<&mut EventWriter<ResourceChanged>>) -> bool {
        let entry = self.resources.entry(resource).or_insert(0.0);
        let old = *entry;
        
        if *entry >= amount {
            *entry -= amount;
            
            if let Some(events) = event_writer {
                events.send(ResourceChanged {
                    resource_type: resource,
                    old_amount: old,
                    new_amount: *entry,
                    source: None,
                });
            }
            
            true
        } else {
            false
        }
    }

    pub fn get(&self, resource: ResourceType) -> f32 {
        *self.resources.get(&resource).unwrap_or(&0.0)
    }
    
    pub fn get_capacity(&self, resource: ResourceType) -> f32 {
        *self.max_capacity.get(&resource).unwrap_or(&f32::INFINITY)
    }
    
    pub fn get_available_space(&self, resource: ResourceType) -> f32 {
        let current = self.get(resource);
        let max = self.get_capacity(resource);
        (max - current).max(0.0)
    }
    
    /// Sets the resource amount directly, bypassing capacity limits
    /// Primarily for debugging and development tools
    pub fn set(&mut self, resource: ResourceType, amount: f32, event_writer: Option<&mut EventWriter<ResourceChanged>>) {
        let old = *self.resources.entry(resource).or_insert(0.0);
        *self.resources.entry(resource).or_insert(0.0) = amount;
        
        if let Some(events) = event_writer {
            events.send(ResourceChanged {
                resource_type: resource,
                old_amount: old,
                new_amount: amount,
                source: None,
            });
        }
    }
    
    /// Sets the maximum capacity for a resource
    pub fn set_capacity(&mut self, resource: ResourceType, capacity: f32) {
        self.max_capacity.insert(resource, capacity);
    }
}

#[derive(Debug, Clone, Component)]
pub struct ResourceSystem {
    pub resources: HashMap<ResourceType, f32>,
    pub max_capacity: HashMap<ResourceType, f32>,
    pub regeneration_rate: f32,
    pub consumption_rate: f32,
}

impl ResourceSystem {
    pub fn new() -> Self {
        let mut max_capacity = HashMap::new();
        max_capacity.insert(ResourceType::Food, 100.0);
        max_capacity.insert(ResourceType::Water, 100.0);
        max_capacity.insert(ResourceType::Energy, 100.0);
        max_capacity.insert(ResourceType::Metal, 100.0);
        max_capacity.insert(ResourceType::Oxygen, 100.0);
        
        Self {
            resources: HashMap::new(),
            max_capacity,
            regeneration_rate: 1.0,
            consumption_rate: 1.0,
        }
    }

    pub fn add(&mut self, resource: ResourceType, amount: f32, event_writer: Option<&mut EventWriter<ResourceChanged>>, source: Option<Entity>) -> f32 {
        let max = *self.max_capacity.get(&resource).unwrap_or(&f32::INFINITY);
        let current = *self.resources.entry(resource).or_insert(0.0);
        let new_amount = (current + amount).min(max);
        let added = new_amount - current;
        
        if added > 0.0 {
            let old = current;
            *self.resources.entry(resource).or_insert(0.0) = new_amount;
            
            if let Some(events) = event_writer {
                events.send(ResourceChanged {
                    resource_type: resource,
                    old_amount: old,
                    new_amount,
                    source,
                });
            }
        }
        
        added
    }

    pub fn consume(&mut self, resource: ResourceType, amount: f32, event_writer: Option<&mut EventWriter<ResourceChanged>>, source: Option<Entity>) -> bool {
        let entry = self.resources.entry(resource).or_insert(0.0);
        let old = *entry;
        
        if *entry >= amount {
            *entry -= amount;
            
            if let Some(events) = event_writer {
                events.send(ResourceChanged {
                    resource_type: resource,
                    old_amount: old,
                    new_amount: *entry,
                    source,
                });
            }
            
            true
        } else {
            false
        }
    }

    pub fn get(&self, resource: ResourceType) -> f32 {
        *self.resources.get(&resource).unwrap_or(&0.0)
    }
    
    pub fn get_capacity(&self, resource: ResourceType) -> f32 {
        *self.max_capacity.get(&resource).unwrap_or(&f32::INFINITY)
    }
    
    pub fn get_available_space(&self, resource: ResourceType) -> f32 {
        let current = self.get(resource);
        let max = self.get_capacity(resource);
        (max - current).max(0.0)
    }
    
    /// Sets the resource amount directly, bypassing capacity limits
    /// Primarily for debugging and development tools
    pub fn set(&mut self, resource: ResourceType, amount: f32, event_writer: Option<&mut EventWriter<ResourceChanged>>, source: Option<Entity>) {
        let old = *self.resources.entry(resource).or_insert(0.0);
        *self.resources.entry(resource).or_insert(0.0) = amount;
        
        if let Some(events) = event_writer {
            events.send(ResourceChanged {
                resource_type: resource,
                old_amount: old,
                new_amount: amount,
                source,
            });
        }
    }
    
    /// Sets the maximum capacity for a resource
    pub fn set_capacity(&mut self, resource: ResourceType, capacity: f32) {
        self.max_capacity.insert(resource, capacity);
    }
}

/// Transfers resources from one system to another
pub fn transfer_resources(
    from: &mut ResourceSystem,
    to: &mut ResourceSystem,
    resource: ResourceType,
    amount: f32,
    event_writer: Option<&mut EventWriter<ResourceChanged>>,
    source: Option<Entity>,
) -> f32 {
    // Calculate how much can actually be transferred
    let available_from = from.get(resource);
    let available_to = to.get_available_space(resource);
    let transfer_amount = amount.min(available_from).min(available_to);
    
    if transfer_amount > 0.0 {
        // Consume from source
        from.consume(resource, transfer_amount, event_writer, source);
        // Add to destination
        to.add(resource, transfer_amount, event_writer, source);
    }
    
    transfer_amount
}

/// System that updates resource regeneration over time
pub fn update_resources(
    time: Res<Time>,
    mut query: Query<&mut ResourceSystem>,
    mut events: EventWriter<ResourceChanged>,
) {
    let delta = time.delta_seconds();
    
    for mut system in query.iter_mut() {
        for resource_type in [ResourceType::Food, ResourceType::Water, ResourceType::Energy, ResourceType::Metal, ResourceType::Oxygen] {
            let metadata = resource_type.metadata();
            
            // Handle regeneration for renewable resources
            if metadata.is_renewable {
                let regeneration = system.regeneration_rate * delta;
                system.add(resource_type, regeneration, Some(&mut events), None);
            }
            
            // Handle decay
            if metadata.decay_rate > 0.0 {
                let decay = metadata.decay_rate * delta;
                let current = system.get(resource_type);
                if current > 0.0 {
                    system.consume(resource_type, decay.min(current), Some(&mut events), None);
                }
            }
        }
    }
}

/// System that processes resource changes and updates UI or other systems
pub fn process_resource_changes(
    mut events: EventReader<ResourceChanged>,
) {
    for event in events.read() {
        // This is a placeholder for future UI updates, logging, or other systems
        // that need to react to resource changes
        info!(
            "Resource changed: {:?} from {} to {} (source: {:?})",
            event.resource_type, event.old_amount, event.new_amount, event.source
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_resource_manager() {
        let mut app = App::new();
        app.add_event::<ResourceChanged>();
        
        let mut manager = ResourceManager::new();
        let mut events = app.world.resource_mut::<Events<ResourceChanged>>();
        let mut writer = EventWriter::new(&mut events);
        
        // Test adding resources
        manager.add(ResourceType::Food, 10.0, Some(&mut writer));
        manager.add(ResourceType::Water, 5.0, Some(&mut writer));
        
        assert_eq!(manager.get(ResourceType::Food), 10.0);
        assert_eq!(manager.get(ResourceType::Water), 5.0);
        
        // Test consuming resources
        assert!(manager.consume(ResourceType::Food, 5.0, Some(&mut writer)));
        assert_eq!(manager.get(ResourceType::Food), 5.0);
        
        // Test consuming more than available
        assert!(!manager.consume(ResourceType::Food, 10.0, Some(&mut writer)));
        assert_eq!(manager.get(ResourceType::Food), 5.0);
        
        // Test capacity limits
        assert_eq!(manager.get_capacity(ResourceType::Food), 1000.0);
        assert_eq!(manager.get_available_space(ResourceType::Food), 995.0);
        
        // Test set method
        manager.set(ResourceType::Food, 20.0, Some(&mut writer));
        assert_eq!(manager.get(ResourceType::Food), 20.0);
        
        // Test set_capacity method
        manager.set_capacity(ResourceType::Food, 50.0);
        assert_eq!(manager.get_capacity(ResourceType::Food), 50.0);
        
        // Verify events were sent
        let mut reader = events.get_reader();
        let events: Vec<&ResourceChanged> = reader.read(&events).collect();
        assert!(events.len() >= 4); // At least 4 events (2 adds, 1 consume, 1 set)
    }

    #[test]
    fn test_resource_system() {
        let mut app = App::new();
        app.add_event::<ResourceChanged>();
        
        let mut system = ResourceSystem::new();
        let mut events = app.world.resource_mut::<Events<ResourceChanged>>();
        let mut writer = EventWriter::new(&mut events);
        
        // Test adding resources
        system.add(ResourceType::Energy, 20.0, Some(&mut writer), None);
        system.add(ResourceType::Metal, 15.0, Some(&mut writer), None);
        
        assert_eq!(system.get(ResourceType::Energy), 20.0);
        assert_eq!(system.get(ResourceType::Metal), 15.0);
        
        // Test consuming resources
        assert!(system.consume(ResourceType::Energy, 10.0, Some(&mut writer), None));
        assert_eq!(system.get(ResourceType::Energy), 10.0);
        
        // Test consuming more than available
        assert!(!system.consume(ResourceType::Energy, 15.0, Some(&mut writer), None));
        assert_eq!(system.get(ResourceType::Energy), 10.0);
        
        // Test capacity limits
        assert_eq!(system.get_capacity(ResourceType::Energy), 100.0);
        assert_eq!(system.get_available_space(ResourceType::Energy), 90.0);
        
        // Test set method
        system.set(ResourceType::Energy, 30.0, Some(&mut writer), None);
        assert_eq!(system.get(ResourceType::Energy), 30.0);
        
        // Test set_capacity method
        system.set_capacity(ResourceType::Energy, 50.0);
        assert_eq!(system.get_capacity(ResourceType::Energy), 50.0);
        
        // Verify events were sent
        let mut reader = events.get_reader();
        let events: Vec<&ResourceChanged> = reader.read(&events).collect();
        assert!(events.len() >= 4); // At least 4 events (2 adds, 1 consume, 1 set)
    }
    
    #[test]
    fn test_resource_transfer() {
        let mut app = App::new();
        app.add_event::<ResourceChanged>();
        
        let mut system1 = ResourceSystem::new();
        let mut system2 = ResourceSystem::new();
        let mut events = app.world.resource_mut::<Events<ResourceChanged>>();
        let mut writer = EventWriter::new(&mut events);
        
        // Add resources to system1
        system1.add(ResourceType::Food, 50.0, Some(&mut writer), None);
        
        // Transfer resources
        let transferred = transfer_resources(&mut system1, &mut system2, ResourceType::Food, 30.0, Some(&mut writer), None);
        
        assert_eq!(transferred, 30.0);
        assert_eq!(system1.get(ResourceType::Food), 20.0);
        assert_eq!(system2.get(ResourceType::Food), 30.0);
        
        // Test transfer with capacity limits
        let transferred2 = transfer_resources(&mut system1, &mut system2, ResourceType::Food, 100.0, Some(&mut writer), None);
        
        // Should only transfer what's available in system1
        assert_eq!(transferred2, 20.0);
        assert_eq!(system1.get(ResourceType::Food), 0.0);
        assert_eq!(system2.get(ResourceType::Food), 50.0);
        
        // Verify events were sent
        let mut reader = events.get_reader();
        let events: Vec<&ResourceChanged> = reader.read(&events).collect();
        assert!(events.len() >= 4); // At least 4 events (1 add, 2 transfers)
    }
    
    #[test]
    fn test_resource_metadata() {
        let food_metadata = ResourceType::Food.metadata();
        assert_eq!(food_metadata.category, ResourceCategory::Basic);
        assert!(food_metadata.is_renewable);
        assert_eq!(food_metadata.decay_rate, 0.1);
        
        let metal_metadata = ResourceType::Metal.metadata();
        assert_eq!(metal_metadata.category, ResourceCategory::Material);
        assert!(!metal_metadata.is_renewable);
        assert_eq!(metal_metadata.decay_rate, 0.0);
    }
} 