use bevy::prelude::*;
use uuid::Uuid;
use crate::agents::agent::Agent;
use crate::agents::message::Message;

pub fn agent_tick_system(
    mut query: Query<(Entity, &mut Agent)>,
    time: Res<Time>,
) {
    // First, process all agent ticks
    for (_, mut agent) in query.iter_mut() {
        agent.tick();
    }

    // Phase 1: Collect all messages to be sent
    let mut messages_to_send: Vec<(Entity, Message)> = Vec::new();
    
    // Get all agent IDs and names first
    let agent_info: Vec<(Entity, Uuid, String)> = query
        .iter()
        .map(|(entity, agent)| (entity, agent.id, agent.name.clone()))
        .collect();
    
    // Create messages without modifying agents
    for (i, (entity, agent_id, agent_name)) in agent_info.iter().enumerate() {
        if !agent_info.is_empty() {
            let next_idx = (i + 1) % agent_info.len();
            let next_agent_id = agent_info[next_idx].1;
            
            let message = Message::new(
                *agent_id,
                next_agent_id,
                format!("Hello from {}!", agent_name),
                time.elapsed_seconds(),
            );
            
            messages_to_send.push((agent_info[next_idx].0, message));
        }
    }
    
    // Phase 2: Deliver all messages
    for (recipient_entity, message) in messages_to_send {
        if let Ok((_, mut recipient)) = query.get_mut(recipient_entity) {
            recipient.message_queue.push_back(message);
        }
    }
}