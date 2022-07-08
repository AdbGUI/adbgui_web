use yew::prelude::*;

#[derive(PartialEq, Eq, Debug, Properties)]
pub struct DropModalProps {
    pub uploading: bool,
}

#[function_component(DropModal)]
pub fn drop_modal(props: &DropModalProps) -> Html {
    let DropModalProps { uploading } = props;
    html!(
        <div class="modal">
            <div class="flex flex-column center expand-all">
                <img alt="APK icon" class="unselectable" src="assets/icons/android.svg"/>
                if *uploading {
                    <span class="unselectable">{"Arrastra aqui un APK para configurar una nueva version"}</span>
                } else {
                    <>
                    </>
                }
            </div>
        </div>
    )
}
