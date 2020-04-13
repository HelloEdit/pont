use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateRoom(String),
    JoinRoom(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    CreatedRoom(String),
    JoinRoom(String, String),
    UnknownRoom(String),
}
