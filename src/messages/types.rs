use std::vec;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{stream::SplitSink, SinkExt};
use rmp_serde::{decode, encode};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::filesystem::query::{list_directory, FileEntry};

pub type ConcurrentWebSocket = SplitSink<WebSocket, axum::extract::ws::Message>;

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum Types {
    Error = 0,
    Query = 1,
}

pub trait FromMsgPack<T: Default + for<'a> Deserialize<'a>> {
    fn from_msgpack(data: &[u8]) -> T {
        match decode::from_slice::<T>(data) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("[err ] Couldn't parse incoming websocket message: {:?}", e);
                T::default()
            }
        }
    }
}

pub trait ProcessMsg<T: Default + for<'a> Deserialize<'a>> {
    /// Process incoming message
    async fn process(&self, websocket: &mut ConcurrentWebSocket);
}

#[derive(Deserialize, Debug)]
pub struct CommonMsg {
    /* Type */
    t: Types,
    /* Arguments */
    a: Vec<u8>,
}

impl Default for CommonMsg {
    fn default() -> Self {
        Self {
            t: Types::Error,
            a: vec![],
        }
    }
}

impl FromMsgPack<CommonMsg> for CommonMsg {}

impl ProcessMsg<CommonMsg> for CommonMsg {
    async fn process(&self, socket: &mut ConcurrentWebSocket) {
        println!("[type] {:?}", self.t);
        match self.t {
            Types::Error => (),
            Types::Query => QueryMsg::from_msgpack(&self.a).process(socket).await,
        }
    }
}

#[derive(Serialize)]
struct CommonResp {
    /* Type */
    t: Types,
    /* Arguments */
    a: Vec<u8>,
}

#[derive(Deserialize)]
pub struct QueryMsg {
    path: String,
}

impl Default for QueryMsg {
    fn default() -> Self {
        Self {
            path: Default::default(),
        }
    }
}

impl FromMsgPack<QueryMsg> for QueryMsg {}
impl QueryMsg {
    pub async fn process(&self, socket: &mut ConcurrentWebSocket) {
        if let Ok(entries) = list_directory(&self.path) {
            let resp = CommonResp {
                t: Types::Query,
                a: match encode::to_vec(&QueryResp { entries }) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("[err ] Failed to encode QueryResp: {:?}", e);
                        return;
                    }
                },
            };

            if let Err(e) = socket
                .send(Message::Binary(match encode::to_vec(&resp) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("[err ] Failed to encode CommonResp: {:?}", e);
                        return;
                    }
                }))
                .await
            {
                eprintln!("[err ] Error sending Query response: {:?}", e);
            }
        } else {
            eprintln!("[err ] Failed to list directory");
        }
    }
}

#[derive(Serialize)]
pub struct QueryResp {
    entries: Vec<FileEntry>,
}
