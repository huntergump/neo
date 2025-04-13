use bevy::prelude::*;
use uuid::Uuid;
use crate::agents::agent::Agent;
use crate::agents::message::Message;
use std::time::Instant;

/// Event fired when an agent completes a tick
#[derive(Event, Debug)]
pub struct AgentTickCompleted {
    pub agent_id: Uuid,
    pub agent_name: String,
    pub tick_count: u32,
    pub duration_ms: f32,
}

/// System that processes agent ticks and message passing
/// 
/// This system:
/// 1. Processes all agent ticks
/// 2. Collects messages to be sent between agents
/// 3. Delivers messages to recipient agents
pub fn agent_tick_system(
    mut query: Query<(Entity, &mut Agent), With<Agent>>,
    time: Res<Time<Fixed>>,
    mut tick_events: EventWriter<AgentTickCompleted>,
) {
    // First, process all agent ticks
    for (entity, mut agent) in query.iter_mut() {
        let start_time = Instant::now();
        
        // Use catch_unwind to prevent a single agent's tick from crashing the entire system
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            agent.tick();
        }));
        
        let duration = start_time.elapsed().as_secs_f32() * 1000.0; // Convert to milliseconds
        
        match result {
            Ok(_) => {
                // Send tick completed event
                tick_events.send(AgentTickCompleted {
                    agent_id: agent.id,
                    agent_name: agent.name.clone(),
                    tick_count: agent.tick_count,
                    duration_ms: duration,
                });
                
                // Log if tick took too long
                if duration > 16.0 { // More than 1 frame at 60 FPS
                    warn!("Agent {} tick took {:.2}ms", agent.name, duration);
                }
            },
            Err(_) => {
                error!("Agent {} (ID: {}) crashed during tick", agent.name, agent.id);
            }
        }
    }

    // Phase 1: Collect all messages to be sent
    let messages_to_send = generate_messages_for_all_agents(&query.to_readonly(), time.elapsed_secs());
    
    // Phase 2: Deliver all messages
    for (recipient_entity, message) in messages_to_send {
        if let Ok((_, mut recipient)) = query.get_mut(recipient_entity) {
            recipient.message_queue.push_back(message);
        } else {
            warn!("Failed to deliver message to agent entity {:?}", recipient_entity);
        }
    }
}

/// Generate messages for all agents
/// 
/// This is a placeholder implementation that creates a simple ring of messages.
/// In a real implementation, this would be controlled by agent behavior trees.
fn generate_messages_for_all_agents(
    query: &Query<(Entity, &Agent), With<Agent>>,
    current_time: f32,
) -> Vec<(Entity, Message)> {
    let mut messages_to_send: Vec<(Entity, Message)> = Vec::new();
    
    // Get all agent IDs and names first
    let agent_info: Vec<(Entity, Uuid, String)> = query
        .iter()
        .map(|(entity, agent)| (entity, agent.id, agent.name.clone()))
        .collect();
    
    // Create messages without modifying agents
    for (i, (_entity, agent_id, agent_name)) in agent_info.iter().enumerate() {
        if !agent_info.is_empty() {
            let next_idx = (i + 1) % agent_info.len();
            let next_agent_id = agent_info[next_idx].1;
            
            let message = Message::new(
                *agent_id,
                next_agent_id,
                format!("Hello from {}!", agent_name),
                current_time,
            );
            
            messages_to_send.push((agent_info[next_idx].0, message));
        }
    }
    
    messages_to_send
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use crate::agents::agent::Agent;

    #[test]
    fn test_agent_message_delivery() {
        // Create a test app
        let mut app = App::new();
        
        // Add required plugins and resources
        app.add_plugins(MinimalPlugins);
        app.add_event::<AgentTickCompleted>();
        app.insert_resource(Time::<Fixed>::from_hz(60.0));
        
        // Spawn two agents
        let agent1_entity = app.world.spawn(Agent::default()).id();
        let agent2_entity = app.world.spawn(Agent::default()).id();
        
        // Run the tick system
        app.update();
        
        // Check that messages were delivered
        let agent1 = app.world.get::<Agent>(agent1_entity).unwrap();
        let agent2 = app.world.get::<Agent>(agent2_entity).unwrap();
        
        // At least one agent should have received a message
        assert!(agent1.message_queue.len() > 0 || agent2.message_queue.len() > 0, 
                "No messages were delivered to either agent");
        
        // Check that tick events were sent
        let mut tick_events = app.world.resource_mut::<Events<AgentTickCompleted>>();
        let mut reader = tick_events.get_reader();
        let events: Vec<&AgentTickCompleted> = reader.read(&tick_events).collect();
        
        assert_eq!(events.len(), 2, "Expected 2 tick events, got {}", events.len());
    }
}