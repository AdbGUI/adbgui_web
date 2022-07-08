#![allow(dead_code)]
use reqwasm::http::Request;
use web_sys::File;

use crate::api::error::Error;

const API_LOGCAT: &str = "api/logcat";
const API_APK: &str = "api/apk";

pub fn get_protocol() -> String {
    let protocol = match std::env::var("USE_SSL") {
        Ok(_) => "https",
        Err(_) => "http",
    };

    protocol.to_string()
}

pub fn get_host() -> String {
    let var = match std::env::var("COMPILATION_PLATFORM") {
        Ok(value) => match value.as_str() {
            "ASPNETCORE" => "ASPNETCORE_PORT",
            &_ => "PORT",
        },
        Err(_) => "PORT",
    };

    let port = match std::env::var(var) {
        Ok(port) => port,
        Err(_) => "8080".to_string(),
    };
    format!("localhost:{}", port)
}

pub fn get_ws() -> String {
    format!("ws://{}/{}", get_host(), API_LOGCAT)
}

pub async fn post_apk_upload(file: Option<File>) -> Result<(), Error> {
    if let Some(apk_file) = file {
        let endpoint = format!("{}://{}/{}", get_protocol(), get_host(), API_APK);
        let response = Request::post(&endpoint)
            .header("Access-Control-Allow-Origin", "*")
            .body(&apk_file)
            .send()
            .await
            .unwrap();
        (response.status() == 200)
            .then(|| ())
            .ok_or(Error::Upload)?;
        log::trace!("uploaded apk filename: {}", apk_file.name());
        Ok(())
    } else {
        Err(Error::Upload)
    }
}

pub async fn get_apk_version() -> Result<String, Error> {
    let endpoint = format!("{}://{}/{}/version", get_protocol(), get_host(), API_APK);

    let response = Request::get(&endpoint)
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await
        .unwrap();
    (response.status() == 200)
        .then(|| "".to_string())
        .ok_or(Error::Empty)?;

    let resp = response.text().await.unwrap_or_default();
    Ok(resp)
}
