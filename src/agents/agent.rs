use bevy::prelude::*;
use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use crate::world::position::Position;
use super::{message::Message, job::Job};

#[derive(Component)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub position: Position,
    pub tick_count: u32,
    pub memory: HashMap<String, String>,
    pub message_queue: VecDeque<Message>,
    pub current_job: Option<Job>,
    pub perception_range: f32,
}

impl Agent {
    pub fn new(name: String, position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            position,
            tick_count: 0,
            memory: HashMap::new(),
            message_queue: VecDeque::new(),
            current_job: Some(Job::Idle),
            perception_range: 10.0, // Default perception range
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        
        // Process any pending messages
        while let Some(message) = self.message_queue.pop_front() {
            self.process_message(message);
        }

        // Update current job if any
        if let Some(job) = self.current_job.clone() {
            self.process_job(&job);
        }

        // Process perceptions (to be implemented)
        self.process_perceptions();

        println!("Agent {} ticked. Tick #: {}", self.name, self.tick_count);
    }

    pub fn process_message(&mut self, message: Message) {
        // Store message in memory
        self.memory.insert(
            format!("message_{}", self.tick_count),
            message.content.clone()
        );
        println!("Agent {} received message: {}", self.name, message.content);
    }

    pub fn process_job(&mut self, job: &Job) {
        println!("Agent {} processing job: {:?}", self.name, job);
        
        // Check if job is complete
        if job.is_complete() {
            self.current_job = Some(Job::Idle);
        }
    }

    pub fn process_perceptions(&mut self) {
        // This will be implemented later to handle:
        // - Visual perception of nearby entities
        // - Sound perception
        // - Resource detection
        // - Threat detection
        // - etc.
    }

    pub fn observe_environment(&self) -> Vec<String> {
        // This will be implemented later to return observations about:
        // - Nearby entities
        // - Available resources
        // - Environmental conditions
        // - etc.
        Vec::new()
    }
}

pub fn spawn_agents(mut commands: Commands) {
    for i in 0..3 {
        let position = Position { x: i as i32, y: 0 };
        let agent = Agent::new(format!("Agent_{}", i), position);
        commands.spawn(agent);
    }
}