use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use serde_json::json;
use std::sync::Arc;

use crate::db::DbConnection;
use crate::models::{WsErrorResponse, WsMessage, WsQueryResponse};

pub struct WsConnection {
    db: &rc<DbConnection>,
}

impl WsConnection {
    pub fn new(db: Arc<DbConnection>) -> Self {
        WsConnection { db }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text(json!({"success": true, "message": "Connected to server"}).to_string());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<WsMessage>(&text) {
                Ok(message) => match self.db.execute_query(&message.query) {
                    Ok(result) => {
                        let response = WsQueryResponse {
                            success: true,
                            result: Some(result),
                            error: None,
                         };
                         ctx.text(serde_json::to_string(&response).unwrap());
                    }
                    Err(e) => {
                        let response = WsErrorResponse {
                            success: false,
                            error: e.to_string(),
                         };
                         ctx.text(serde_json::to_string(&response).unwrap());
                    }
                },
                Err(e) => {
                    let response = WsErrorResponse {
                        success: false,
                        error: format!("Invalid message format: {}", e),
                    };
                },
                Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
                Ok(ws::Message::Pong(_)) => {}
                Ok(ws::Message::Close(reason)) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => ctx.stop(),
            }
        }
    }
}