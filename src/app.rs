use yew::prelude::*;
use yew_hooks::{use_drop_with_options, UseDropOptions};

use crate::components::drop_modal::DropModal;
use crate::components::logcat_shell::LogcatShell;
use crate::components::sidebar::{Sidebar, SidebarBtnProps};
use yew_icons::{Icon, IconId};

#[function_component(App)]
pub fn app() -> Html {
    // Sidebar options
    let mut options = Vec::new();
    let c = Children::new(vec![
        html! {<Icon icon_id={IconId::FeatherHome} width={"1.3em".to_owned()} height={"1.3em".to_owned()} />},
    ]);
    options.push(SidebarBtnProps {
        icon: c,
        name: "Home".to_string(),
        on_click: Callback::from(|_| println!("Click Home")),
    });

    // Drop apk
    let node = use_node_ref();
    let modal_state = use_state(|| false);

    {
        let node = node.clone();
        let modal_state = modal_state.clone();

        use_drop_with_options(
            node,
            UseDropOptions {
                ondragenter: Some(Box::new(move |_| {
                    modal_state.set(true);
                })),
                ..Default::default()
            },
        )
    };

    let on_close = {
        let modal_state = modal_state.clone();
        Callback::from(move |_| modal_state.set(false))
    };

    html! {
        <main>
            <Sidebar {options} />
            <div class="main_content">
                <div class="circle_android" ref={node}>
                    <div class="wave delay1"></div>
                    <div class="wave delay2"></div>
                    <div class="wave delay3"></div>
                    <div class="wave delay4"></div>
                    <img alt="APK icon" class="unselectable" src="assets/icons/android.svg"/>
                    <span class="unselectable">{"6.2.7"}</span>
                </div>
                <LogcatShell />
            </div>
            if *modal_state {
                <DropModal on_close={on_close} />
            }
        </main>
    }
}
