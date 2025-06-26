use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::start;
use std::sync::Arc;

mod db;
mod errors;
mod models;
mod ws;

use crate::db::DbConnection;
use crate::ws::WsConnection;