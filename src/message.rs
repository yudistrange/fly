use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Debug, Deserialize, Serialize)]
struct Body {
    #[serde(flatten)]
    message_type: MessageType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageType {
    Init {
        msg_id: u32,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        msg_id: u32,
        in_reply_to: u32,
    },
    Echo {
        msg_id: u32,
        echo: String,
    },
    EchoOk {
        msg_id: u32,
        in_reply_to: u32,
        echo: String,
    },
}

impl Message {
    pub fn new(source: String, destination: String, msg: MessageType, _msg_id: u32) -> Message {
        Message {
            src: source.clone(),
            dest: destination,
            body: Body { message_type: msg },
        }
    }

    pub fn read() -> Result<Message, serde_json::Error> {
        let mut _de = serde_json::Deserializer::from_reader(io::stdin());
        Message::deserialize(&mut _de)
    }

    pub fn write(&self, writer: io::Stdout) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, self) //
    }

    pub fn message_type(&self) -> MessageType {
        return self.body.message_type.clone();
    }

    pub fn source(&self) -> String {
        self.src.clone()
    }

    pub fn dest(&self) -> String {
        self.dest.clone()
    }
}

impl fmt::Display for Message {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "src: {}\ndest: {}\nbody: {}",
            self.src, self.dest, self.body
        )
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
