use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use my_lib::libinfo;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging only during debug builds 
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        fn hello_text() -> &'static str {
            "Yes, feature is enabled (in app)!"
        }
    } else {
        fn hello_text() -> &'static str {
            "Nope, feature is disabled (in app)!"
        }
    }
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some(hello_text()));
    body.append_child(&p)?;

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some(libinfo::hello_text()));
    body.append_child(&p)?;
    Ok(())
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
