#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Job {
    Idle,
    Move { target_x: i32, target_y: i32 },
    Gather { resource_type: String },
    Build { structure_type: String },
    Interact { target_id: String },
}

impl Job {
    pub fn is_complete(&self) -> bool {
        match self {
            Job::Idle => true,
            Job::Move { target_x: _, target_y: _ } => false, // Will be implemented with position checking
            Job::Gather { resource_type: _ } => false, // Will be implemented with inventory checking
            Job::Build { structure_type: _ } => false, // Will be implemented with construction checking
            Job::Interact { target_id: _ } => false, // Will be implemented with interaction checking
        }
    }
} 