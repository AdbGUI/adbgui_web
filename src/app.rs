use web_sys::File;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::api::post_apk_upload;
use crate::components::drop_modal::DropModal;
use crate::components::logcat_shell::LogcatShell;
use crate::components::sidebar::{Sidebar, SidebarBtnProps};
use yew_icons::{Icon, IconId};

#[derive(Default)]
pub struct ApkDropState {
    pub show_drop_modal: bool,
    pub uploading: bool,
    pub apk: Option<File>,
    pub apk_name: String,
}

#[function_component(App)]
pub fn app() -> Html {
    // Sidebar options
    let mut options = Vec::new();
    let c = Children::new(vec![
        html! {<Icon icon_id={IconId::FeatherHome} width={"1.3em".to_owned()} height={"1.3em".to_owned()} />},
    ]);
    options.push(SidebarBtnProps {
        icon: c.clone(),
        name: "Home".to_string(),
        on_click: Callback::from(|_| println!("Click Home")),
    });

    // Drop apk
    let apk_state = use_state(|| ApkDropState::default());
    let node = use_node_ref();
    let upload = {
        let apk_state = apk_state.clone();

        use_async(async move {
            post_apk_upload(apk_state.apk.clone()).await
        })
    };
    let _drop_state = {
        let apk_state = apk_state.clone();
        let over_state = apk_state.clone();
        let leave_state = apk_state.clone();
        let upload = upload.clone();

        use_drop_with_options(
            node.clone(),
            UseDropOptions {
                ondragenter: Some(Box::new(move |_| {
                    over_state.set(ApkDropState {
                        show_drop_modal: true,
                        apk: None,
                        apk_name: over_state.apk_name.clone(),
                        ..*over_state
                    });
                })),
                ondragleave: Some(Box::new(move |_| {
                    leave_state.set(ApkDropState {
                        show_drop_modal: false,
                        apk: None,
                        apk_name: "".to_string(),
                        ..*leave_state
                    });
                })),
                onfiles: Some(Box::new(move |files, _data_transfer| {
                    // Process files or data_transfer
                    if let Some(apk) = files.iter().filter(|f| f.name().ends_with("apk")).next() {
                        apk_state.set(ApkDropState {
                            show_drop_modal: false,
                            uploading: true,
                            apk: Some(apk.clone()),
                            apk_name: apk.name().clone(),
                        });
                        upload.run();
                    } else {
                        apk_state.set(ApkDropState {
                            show_drop_modal: false,
                            uploading: false,
                            apk: None,
                            apk_name: "".to_string(),
                        });
                    }
                })),
                ..Default::default()
            },
        )
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
            if apk_state.show_drop_modal {
                <DropModal uploading={apk_state.uploading} />
            }
        </main>
    }
}
