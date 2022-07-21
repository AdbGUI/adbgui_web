use yew::prelude::*;

#[derive(PartialEq, Debug, Properties)]
pub struct AdbButtonProps {
    pub type_btn: String,
    pub text: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(AdbButton)]
pub fn adb_button(props: &AdbButtonProps) -> Html {
    let AdbButtonProps {
        type_btn,
        text,
        onclick,
    } = &props;

    let mut classes = Classes::new();
    classes.push("flex");
    classes.push("center");
    classes.push("unselectable");
    classes.push("btn");
    classes.push(format!("btn_{type_btn}"));

    html!(
        <div class={classes} onclick={onclick}>
            {text}
        </div>
    )
}
