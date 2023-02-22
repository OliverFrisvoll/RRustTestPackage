use extendr_api::prelude::*;
use isahc::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Testing if i can send http requests
/// @export
#[extendr]
fn http_req(url: String) -> String {
    isahc::get(url)
        .unwrap()
        .text()
        .unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod RRustTestPackage;
    fn hello_world;
    fn http_req;
}
