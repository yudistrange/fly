use serde::Serialize;
use serde_json;

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
        log::info!("Generating a message struct");
        let m = Message::new(self.id.clone(), destination, msg_type, self.current_msg_id);

        let _ = serde_json::to_writer(std::io::stdout(), &m);

        match m.write(std::io::stdout()) {
            Ok(_) => log::info!("Wrote message to stdout"),
            Err(e) => log::error!("Message writing failed with error: {}", e.to_string()),
        }
    }
}
