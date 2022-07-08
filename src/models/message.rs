#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TypeMessage {
    Error,
    Warning,
    Info,
}

impl Default for TypeMessage {
    fn default() -> Self {
        TypeMessage::Info
    }
}

impl ToString for TypeMessage {
    fn to_string(&self) -> String {
        match self {
            TypeMessage::Error => String::from("Error"),
            TypeMessage::Warning => String::from("Warn"),
            TypeMessage::Info => String::from("Info"),
        }
    }
}
