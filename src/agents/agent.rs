use bevy::prelude::*;
use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use crate::world::position::Position;
use super::{message::Message, job::Job};
use crate::SimulationConfig;
use rand::random;

/// Represents an agent in the simulation
/// 
/// Agents are autonomous entities that can:
/// - Move around the world
/// - Process messages
/// - Execute jobs
/// - Perceive their environment
/// - Learn from experience
#[derive(Component)]
pub struct Agent {
    /// Unique identifier for the agent
    pub id: Uuid,
    /// Name of the agent
    pub name: String,
    /// Current position in the world
    pub position: Vec2,
    /// Number of ticks this agent has processed
    pub tick_count: u32,
    /// Memory storage for the agent
    pub memory: HashMap<String, String>,
    /// Queue of messages waiting to be processed
    pub message_queue: VecDeque<Message>,
    /// Current job being executed
    pub current_job: Option<Job>,
    /// Range at which the agent can perceive the environment
    pub perception_range: f32,
    /// Current velocity vector
    pub velocity: Vec2,
    /// Current energy level
    pub energy: f32,
    /// Age of the agent in seconds
    pub age: f32,
}

impl Default for Agent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            position: Vec2::ZERO,
            tick_count: 0,
            memory: HashMap::new(),
            message_queue: VecDeque::new(),
            current_job: Some(Job::Idle),
            perception_range: 10.0,
            velocity: Vec2::ZERO,
            energy: 100.0,
            age: 0.0,
        }
    }
}

impl Agent {
    /// Creates a new agent with the given name and position
    pub fn new(name: String, position: Position) -> Self {
        let id = Uuid::new_v4();
        info!("Created new agent: {} with ID: {}", name, id);
        Self {
            id,
            name,
            position: Vec2::new(position.x as f32, position.y as f32),
            tick_count: 0,
            memory: HashMap::new(),
            message_queue: VecDeque::new(),
            current_job: Some(Job::Idle),
            perception_range: 10.0,
            velocity: Vec2::ZERO,
            energy: 100.0,
            age: 0.0,
        }
    }

    /// Processes a single tick for this agent
    pub fn tick(&mut self) {
        self.tick_count += 1;
        
        // Process any pending messages
        let message_count = self.message_queue.len();
        while let Some(message) = self.message_queue.pop_front() {
            self.process_message(message);
        }
        if message_count > 0 {
            debug!("Agent {} processed {} messages", self.name, message_count);
        }

        // Update current job if any
        if let Some(job) = self.current_job.clone() {
            self.process_job(&job);
        }

        // Process perceptions (to be implemented)
        self.process_perceptions();

        if self.tick_count % 100 == 0 {
            info!("Agent {} ticked. Tick #: {}", self.name, self.tick_count);
        } else {
            debug!("Agent {} ticked. Tick #: {}", self.name, self.tick_count);
        }
    }

    /// Processes a message received by the agent
    pub fn process_message(&mut self, message: Message) {
        // Store message in memory
        self.memory.insert(
            format!("message_{}", self.tick_count),
            message.content.clone()
        );
        debug!("Agent {} received message: {}", self.name, message.content);
    }

    /// Processes the current job
    pub fn process_job(&mut self, job: &Job) {
        debug!("Agent {} processing job: {:?}", self.name, job);
        
        // Check if job is complete
        if job.is_complete() {
            info!("Agent {} completed job: {:?}", self.name, job);
            self.current_job = Some(Job::Idle);
        }
    }

    /// Processes environmental perceptions
    pub fn process_perceptions(&mut self) {
        // This will be implemented later to handle:
        // - Visual perception of nearby entities
        // - Sound perception
        // - Resource detection
        // - Threat detection
        // - etc.
    }

    /// Observes the environment and returns a list of observations
    pub fn observe_environment(&self) -> Vec<String> {
        // This will be implemented later to return observations about:
        // - Nearby entities
        // - Available resources
        // - Environmental conditions
        // - etc.
        Vec::new()
    }
}

/// Spawns agents based on the simulation configuration
pub fn spawn_agents(
    mut commands: Commands,
    config: Res<SimulationConfig>,
) {
    for _ in 0..config.agent_count {
        commands.spawn(Agent::default());
    }
    info!("Spawned {} agents", config.agent_count);
}

/// Updates all agents based on the current time and simulation speed
pub fn update_agents(
    time: Res<Time>,
    config: Res<SimulationConfig>,
    mut query: Query<&mut Agent>,
) {
    let dt = time.delta_secs() * config.simulation_speed as f32;
    
    for mut agent in query.iter_mut() {
        // Store velocity in a local variable to avoid borrowing issues
        let velocity = agent.velocity;
        agent.position += velocity * dt;
        
        // Update agent age
        agent.age += dt;
        
        // Consume energy over time
        agent.energy -= dt * 0.1;
        
        // Basic movement behavior
        if velocity.length() < 1.0 {
            agent.velocity = Vec2::new(
                random::<f32>() * 2.0 - 1.0,
                random::<f32>() * 2.0 - 1.0,
            );
        }
    }
}