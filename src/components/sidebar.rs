use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SidebarBtnProps {
    pub icon: Children,
    pub name: String,
    pub on_click: Callback<MouseEvent>,
}

#[derive(PartialEq, Properties)]
pub struct SidebarProps {
    pub options: Vec<SidebarBtnProps>,
}

#[function_component(ButtonSidebar)]
pub fn sidebar_btn(props: &SidebarBtnProps) -> Html {
    let SidebarBtnProps { icon, name: _, on_click: onclick } = &props;

    html!{
        <div class="sidebar_button" {onclick}> {icon.clone()} </div>
    }
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let SidebarProps { options } = &props;
    html! {
        <div class={"sidebar"}>
        {for options.iter().map(|o| {
                html!{
                    <ButtonSidebar icon={o.icon.clone()} name={o.name.clone()} on_click={o.on_click.clone()} />
                }
            })
        }
        </div>
    }
}
