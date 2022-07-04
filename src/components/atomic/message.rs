use crate::models::message::TypeMessage;
use serde::{Serialize, Deserialize};
use yew::{function_component, html, Properties, Classes};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Properties)]
pub struct MessageProps {
    pub type_message: TypeMessage,
    pub content: String,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    let mut span_type_class = Classes::from("message");
    match props.type_message {
        TypeMessage::Error => span_type_class.push("error"),
        TypeMessage::Warning => span_type_class.push("warning"),
        TypeMessage::Info => span_type_class.push("info"),
    };

    html! {
        <div class={"logcat_message"}>
            <span class={span_type_class}>{&props.type_message.to_string()}</span>
            <span class="message_content">{&props.content}</span>
        </div>
    }
}
