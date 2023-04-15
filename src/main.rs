mod message;
mod node;

use crate::message::{Message, MessageType};
use crate::node::Node;
use serde_json;
use std::collections::HashMap;

fn main() -> serde_json::Result<()> {
    let mut node_list: HashMap<String, Node> = HashMap::new();
    loop {
        match Message::read() {
            Ok(msg) => handle_message(&mut node_list, msg),
            Err(e) => {
                eprintln!("Failed to parse msg with error: {}", e.to_string());
                break;
            }
        }
    }
    Ok(())
}

fn handle_message(node_list: &mut HashMap<String, Node>, msg: Message) {
    let node_id = msg.dest();
    let origin = msg.source();
    match msg.message_type() {
        MessageType::Init {
            msg_id,
            node_id,
            node_ids,
        } => match node_list.get(&node_id) {
            Some(_) => eprintln!("Node already registered with id: {}", node_id),
            None => {
                let n = Node::new(node_id, node_ids);
                n.send(
                    origin,
                    MessageType::InitOk {
                        msg_id: 5,
                        in_reply_to: msg_id,
                    },
                );
                node_list.insert(n.id.clone(), n);
            }
        },
        MessageType::Echo { msg_id, echo } => match node_list.get(&node_id) {
            Some(n) => n.send(
                origin,
                MessageType::EchoOk {
                    msg_id: 6,
                    in_reply_to: msg_id,
                    echo,
                },
            ),
            None => eprintln!("No node registered with id: {}", node_id),
        },
        _ => {}
    }
}
