#![allow(dead_code)]

use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AdjustableContainerDirection {
    Horizontal,
    Vertical,
    Both
}

#[derive(Default, Debug, PartialEq, Properties)]
pub struct AdjustableContainerProps {
    #[prop_or_default]
    pub children: Children,
    pub direction: AdjustableContainerDirection,
}

impl Default for AdjustableContainerDirection {
    fn default() -> Self {
        AdjustableContainerDirection::Vertical
    }
}

#[function_component(AdjustableContainer)]
pub fn adjustable_container(props: &AdjustableContainerProps) -> Html {
    let AdjustableContainerProps { direction: _, children } = &props;

    let can_drag = use_state(|| false);
    let drag = use_state(|| 0);

    let container_ref = NodeRef::default();

    let onmousedown = {
        let state = drag.clone();
        let can_drag = can_drag.clone();
        Callback::from(move |m: MouseEvent| {
            state.set(m.y());
            can_drag.set(true);
        })
    };

    {
        let state = drag.clone();
        let can_drag = can_drag.clone();
        let container_ref = container_ref.clone();
        use_event_with_window("mousemove", move |e: MouseEvent| {
            if *can_drag {
                let container = container_ref.cast::<HtmlElement>().unwrap();
                let offset_height = container.offset_height();
                let new_height = offset_height - (e.client_y() - (*drag));
                container.style().set_property("height", format!("{}px", new_height).as_str()).unwrap();
                state.set(e.client_y());
            }
        });
    }

    {
        let can_drag = can_drag.clone();
        use_event_with_window("mouseup", move |_e: MouseEvent| {
            if *can_drag {
                let document = web_sys::window().unwrap().document().unwrap();
                document.set_onmousemove(None);
                document.set_onmouseup(None);
            }
            can_drag.set(false);
        });
    }

    html! {
        <div ref={container_ref} class="logcat_container">
            <div class="resizer" {onmousedown} />
            {for children.iter()}
        </div>
    }
}
