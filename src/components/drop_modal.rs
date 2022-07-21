use yew::prelude::*;
use yew_hooks::{use_drop_with_options, UseDropOptions};
use yew_icons::{Icon, IconId};

use crate::components::atomic::button::AdbButton;

#[derive(Default, Debug)]
struct DropModalApkState {
    pub file_name: String,
}

#[derive(PartialEq, Debug, Properties)]
pub struct DropModalProps {
    pub on_close: Callback<MouseEvent>,
}

#[function_component(DropModal)]
pub fn drop_modal(props: &DropModalProps) -> Html {
    let DropModalProps { on_close } = props;
    let state = use_state(DropModalApkState::default);
    let show_form = use_state(|| false);
    let node = use_node_ref();

    // TODO: implement
    // let upload = {
    //     let apk_state = state.clone();
    //
    //     use_async(async move { post_apk_upload(apk_state.clone()).await })
    // };

    let drop_state = {
        // let upload = upload;
        let show_form = show_form.clone();
        let state = state.clone();

        use_drop_with_options(
            node.clone(),
            UseDropOptions {
                onfiles: Some(Box::new(move |files, _data_transfer| {
                    if let Some(file) = files.iter().find(|f| f.name().ends_with("apk")) {
                        show_form.set(true);
                        state.set(DropModalApkState {
                            file_name: file.name(),
                        });
                    }
                })),
                ..Default::default()
            },
        )
    };

    let acept_callback = Callback::from(|_| {});

    html!(
        <div ref={node} class="modal">
            <div class="close_button" onclick={on_close}>
                <Icon icon_id={IconId::OcticonsX24} width={"1.3em".to_owned()} height={"1.3em".to_owned()} />
            </div>
            <div class="flex flex-column center expand-all">
                <img alt="APK icon" class="unselectable" src="assets/icons/android.svg"/>
                if *drop_state.over {
                    <span class="unselectable">{"Arrastra aqui un APK para configurar una nueva version"}</span>
                }
                if *show_form {
                    <div class="flex flex-column center">
                        <span class="unselectable">{&state.file_name}</span>
                        <input type="text" placeholder="Nombre, Ej.- Mi Super App" value="" />
                        <input type="text" placeholder="Version, Ej.- 6.8.2.rc.1.0" value="" />
                        <div class="flex flex-row" style="margin: 20px 0">
                            <AdbButton onclick={on_close} text="Cancelar" type_btn="secondary" />
                            <AdbButton onclick={acept_callback} text="Aceptar" type_btn="primary" />
                        </div>
                    </div>
                }
            </div>
        </div>
    )
}
