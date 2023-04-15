use serde::Serialize;

use crate::message::{Message, MessageType};

#[derive(Debug, Serialize)]
pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    current_msg_id: u32,
}

impl Node {
    pub fn new(id: String, node_ids: Vec<String>) -> Node {
        Node {
            id,
            node_ids,
            current_msg_id: 0,
        }
    }

    pub fn send(&self, destination: String, msg_type: MessageType) {
        let m = Message::new(self.id.clone(), destination, msg_type, self.current_msg_id);
        m.write(std::io::stdout()).unwrap();
    }
}
