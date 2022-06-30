#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TypeMessage {
    Error,
    Warning,
    Info
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
