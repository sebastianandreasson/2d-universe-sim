use cfg_if::cfg_if;
use js_sys::Function;
use wasm_bindgen::JsValue;

pub fn rand_int(n: i32) -> i32 {
    (js_sys::Math::random() * n as f64) as i32
}

pub fn rand_dir() -> i32 {
    let i = rand_int(1000);
    (i % 3) - 1
}

pub fn run_js(js: &str) -> Result<JsValue, JsValue> {
    Function::new_no_args(js).call0(&JsValue::NULL)
}

pub fn get_pkg_js_uri() -> String {
    let href = run_js("return location.href;")
        .unwrap()
        .as_string()
        .unwrap();
    format!("{}bundle.js", href)
}

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
         pub fn set_panic_hook() {}
    }
}
