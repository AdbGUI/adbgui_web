use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

macro_rules! api_get {
    ($fn_name: tt, $path: ty, $obj: ty) => (
        pub async fn get_$fn_name () -> Result<$obj, FetchError> {
            let host = crate::api::get_host();
            let mut opts = RequestInit::new();
                opts.method("GET");
                opts.mode(RequestMode::Cors);
            let req = web_sys::Request::new_with_str_and_init(format!("{}/{}", host, $path).as_str(), &opts);
        }
    )
}
