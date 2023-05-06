use std::collections::HashMap;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyMessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<usize>,
    #[serde(rename = "type")]
    pub msg_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestMessageBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub node_ids: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<usize>,
    pub echo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<HashMap<String, Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
    pub src: String,
    pub dest: String,
    pub body: RequestMessageBody,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub src: String,
    pub dest: String,
    pub body: ReplyMessageBody,
}
