use crate::api::get_ws;
use crate::components::atomic::adjustable_container::{
    AdjustableContainer, AdjustableContainerDirection,
};
use yew::prelude::*;
use yew_hooks::{use_list, use_web_socket_with_options, UseWebSocketOptions};

use super::atomic::message::{Message, MessageProps};

#[function_component(LogcatShell)]
pub fn logcat_shell() -> Html {
    let messages = use_list(vec![]);

    {
        let messages = messages.clone();
        use_web_socket_with_options(
            get_ws(),
            UseWebSocketOptions {
                onmessage: Some(Box::new(move |message| {
                    let msg =
                        serde_json::from_str::<MessageProps>(message.as_str()).unwrap_or_default();
                    messages.push(msg);
                })),
                ..Default::default()
            },
        )
    };

    html! {
        <AdjustableContainer direction={AdjustableContainerDirection::Vertical}>
            <div class={ if messages.current().len() == 0 { "connecting_container" } else { "connecting_container hidden" }}>
                <span class="loader"></span>
            </div>
            <div class={ if messages.current().len() > 0 { "container_messages" } else { "container_messages hidden" }}>
            {
                for messages.current().iter().map(|m| {
                    html! {
                        <Message type_message={m.type_message.clone()} content={m.content.clone()} />
                    }
                })
            }
            </div>
        </AdjustableContainer>
    }
}
