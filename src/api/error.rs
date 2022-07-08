#![allow(dead_code)]
use std::num::ParseIntError;

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Upload failed")]
    Upload,

    #[error("Connection closed")]
    WsClosed,

    #[error("Connection error")]
    WsError,

    #[error("Empty response")]
    Empty,

    #[error("Not logged in")]
    NotLoggedIn,

    #[error("Invalid session id")]
    InvalidSession(#[from] ParseIntError),
}
