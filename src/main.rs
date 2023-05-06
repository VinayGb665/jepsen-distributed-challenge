use std::{
    io::{stdin, stdout, Result, Write},
    sync::Mutex,
};

use models::{ReplyMessageBody, Request, RequestMessageBody, Response};
use serde_json::{from_str, json};
pub mod models;

struct Node {
    id: String,
    messages: Mutex<Vec<i32>>,
    neigbours: Vec<String>,
}

impl Node {
    fn new(id: String) -> Self {
        Self {
            id,
            neigbours: Vec::new(),
            messages: Mutex::new(Vec::new()),
        }
    }

    fn send_broadcast_to_neighbour(&self, node_id: String, message: i32) -> Result<()> {
        let req = Request {
            id: None,
            src: self.id.clone(),
            dest: node_id,
            body: RequestMessageBody {
                node_id: None,
                msg_type: "broadcast".to_string(),
                node_ids: None,
                msg_id: None,
                echo: None,
                topology: None,
                message: Some(message),
            },
        };

        let stdout = stdout();
        let json_reply = json!(req).to_string();
        let mut handler = stdout.lock();
        handler.write(json_reply.as_bytes())?;
        handler.write("\n".as_bytes())?;
        handler.flush()?;
        Ok(())
    }
    fn send_response(&self, resp: Response) -> Result<()> {
        let stdout = stdout();
        let json_reply = json!(resp).to_string();
        let mut handler = stdout.lock();
        handler.write(json_reply.as_bytes())?;
        handler.write("\n".as_bytes())?;
        handler.flush()?;
        Ok(())
    }

    fn parse_request(&mut self, buffer: &String) -> Result<Option<Response>> {
        eprintln!("Recieved : {buffer:?}");
        let json_parsed_input: Request = from_str(&buffer)?;
        let message_type = json_parsed_input.body.msg_type;
        eprintln!("Recieved messsage of type {message_type}");
        let response = match message_type.as_str() {
            "init" => {
                self.id = json_parsed_input.body.node_id.unwrap();
                Some(Response {
                    src: json_parsed_input.dest.clone(),
                    dest: json_parsed_input.src.clone(),
                    body: ReplyMessageBody {
                        node_id: Some(json_parsed_input.dest.clone()),
                        messages: None,
                        in_reply_to: json_parsed_input.body.msg_id,
                        msg_id: json_parsed_input.body.msg_id,
                        msg_type: "init_ok".to_owned(),
                        echo: None,
                    },
                })
            }

            "echo" => Some(Response {
                src: json_parsed_input.dest.clone(),
                dest: json_parsed_input.src.clone(),
                body: ReplyMessageBody {
                    node_id: None,
                    messages: None,
                    echo: json_parsed_input.body.echo,
                    in_reply_to: json_parsed_input.body.msg_id,
                    msg_id: json_parsed_input.body.msg_id,
                    msg_type: "echo_ok".to_owned(),
                },
            }),

            "topology" => {
                let topology = json_parsed_input.body.topology.unwrap();
                let neighbours = topology.get(&self.id.clone()).unwrap();
                eprintln!("Topology says my neigbours are {neighbours:?}");
                self.neigbours = neighbours.clone();

                Some(Response {
                    src: json_parsed_input.dest.clone(),
                    dest: json_parsed_input.src.clone(),
                    body: ReplyMessageBody {
                        node_id: None,
                        messages: None,
                        echo: json_parsed_input.body.echo,
                        in_reply_to: json_parsed_input.body.msg_id,
                        msg_id: json_parsed_input.body.msg_id,
                        msg_type: "topology_ok".to_owned(),
                    },
                })
            }

            "broadcast" => {
                let message = json_parsed_input.body.message.unwrap();
                let msg_id = json_parsed_input.body.msg_id;
                eprintln!("Got broadcasted {message}");
                let mut messages = self.messages.lock().unwrap();
                if !messages.contains(&message) {
                    messages.push(message);
                    for node in self.neigbours.clone() {
                        self.send_broadcast_to_neighbour(node, message)?;
                    }
                }
                if msg_id.is_none() {
                    return Ok(None);
                }
                Some(Response {
                    src: json_parsed_input.dest.clone(),
                    dest: json_parsed_input.src.clone(),
                    body: ReplyMessageBody {
                        node_id: None,
                        messages: None,
                        echo: json_parsed_input.body.echo,
                        in_reply_to: json_parsed_input.body.msg_id,
                        msg_id: json_parsed_input.body.msg_id,
                        msg_type: "broadcast_ok".to_owned(),
                    },
                })
            }
            "read" => {
                let messages = self.messages.lock().unwrap();
                Some(Response {
                    src: json_parsed_input.dest.clone(),
                    dest: json_parsed_input.src.clone(),
                    body: ReplyMessageBody {
                        node_id: None,
                        messages: Some(messages.clone()),
                        echo: json_parsed_input.body.echo,
                        in_reply_to: json_parsed_input.body.msg_id,
                        msg_id: json_parsed_input.body.msg_id,
                        msg_type: "read_ok".to_owned(),
                    },
                })
            }
            _ => {
                eprintln!("Invalid message type recieved, p A nic");
                panic!()
            }
        };
        Ok(response)
    }
}

fn main() -> Result<()> {
    let mut node = Node::new("n1".to_string());
    let mut buffer = String::new();
    loop {
        let inp = stdin();
        inp.read_line(&mut buffer).unwrap();
        let response = node.parse_request(&buffer)?;
        if response.is_some() {
            node.send_response(response.unwrap())?;
        }
        buffer.clear();
    }
}
