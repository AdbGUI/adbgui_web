use yew::prelude::*;

use yew_icons::{Icon, IconId};
use crate::components::sidebar::{Sidebar, SidebarBtnProps};

#[function_component(App)]
pub fn app() -> Html {
    let mut options = Vec::new();
    let c = Children::new(vec![
        html!{<Icon icon_id={IconId::FeatherHome} width={"1.3em".to_owned()} height={"1.3em".to_owned()} />},
    ]);
    options.push(SidebarBtnProps {
        icon: c.clone(),
        name: "Home".to_string(),
        on_click: Callback::from(|_| println!("Click Home"))
    });
    html! {
        <main>
            <Sidebar {options} />
            <div class="main_content">
                <div class="circle_android">
                    <div class="wave delay1"></div>
                    <div class="wave delay2"></div>
                    <div class="wave delay3"></div>
                    <Icon icon_id={IconId::FeatherHome} width={"5.3em".to_owned()} height={"5.3em".to_owned()} />     <div class="wave delay4"></div>
                    <span>{"6.2.7"}</span>
                </div>
            </div>
        </main>
    }
}
