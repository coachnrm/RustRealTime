use derive_more::Display;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum WsError {
    #[display("Database error: {}", _0)]
    Database(String),
    #[display("Connection error: {}", _0)]
    Connection(String),
}

impl Error for WsError {}