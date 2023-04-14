use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Node {
    pub id: u32,
    pub current_msg_id: u32,
}
