
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Null,
    Int(i64),
    Real(f64),
    String(String),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
    pub query: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WsQueryResponse {
    pub success: bool,
    pub result: Option<QueryResult>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsErrorResponse {
    pub success: bool,
    pub error: String,
}