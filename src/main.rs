mod node;

use crate::node::Node;
use std::io;

fn main() {
    let x = Node {
        id: 1,
        current_msg_id: 1,
    };

    match serde_json::to_writer(io::stdout(), &x) {
        Ok(_) => println!("\nSucess"),
        Err(_) => println!("\nError"),
    }
}
