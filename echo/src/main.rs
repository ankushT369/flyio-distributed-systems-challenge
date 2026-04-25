use std::io::{self, BufRead};
use serde_json::{Value, json};

/// State Machines for handling Message types
enum MsgType {
    Init,
    Echo,
    Unknown,
}

impl From<&str> for MsgType {
    fn from(s: &str) -> Self {
        match s {
            "init" => MsgType::Init,
            "echo" => MsgType::Echo,
            _ => MsgType::Unknown,
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;

        let msg: Value = serde_json::from_str(&line).unwrap();

        let msg_type = msg["body"]["type"].as_str().unwrap().into();
        let src = msg["src"].as_str().unwrap();
        let dest = msg["dest"].as_str().unwrap();
        let msg_id = msg["body"]["msg_id"].as_u64().unwrap_or(0);

        let mut body = json!({});

        match msg_type {
            MsgType::Init => {
                body["type"] = json!("init_ok");
                body["in_reply_to"] = json!(msg_id);
            }
            MsgType::Echo => {
                body["type"] = json!("echo_ok");
                body["msg_id"] = json!(msg_id); 
                body["in_reply_to"] = json!(msg_id);
                body["echo"] = msg["body"]["echo"].clone();
            }
            MsgType::Unknown => {
                body["type"] = json!("error");
                body["in_reply_to"] = json!(msg_id);
            }
        }

        let response = json!({
            "src": dest,
            "dest": src,
            "body": body
        });

        println!("{}", response);
    }

    Ok(())
}
