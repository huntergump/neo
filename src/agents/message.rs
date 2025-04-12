use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Message {
    pub from: Uuid,
    pub to: Uuid,
    pub content: String,
    pub timestamp: f32,
}

impl Message {
    pub fn new(from: Uuid, to: Uuid, content: String, timestamp: f32) -> Self {
        Self {
            from,
            to,
            content,
            timestamp,
        }
    }
} 