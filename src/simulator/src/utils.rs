use cfg_if::cfg_if;

pub fn rand_to(n: u8) -> u8 {
    (js_sys::Math::random() * (n as f64)).floor() as u8
}

pub fn rand_dir_with_bias() -> i8 {
    if (js_sys::Math::random() as f64) > 0.66 {
        return 1;
    }
    return -1;
}

pub fn rand_dir() -> i8 {
    if (js_sys::Math::random() as f64) > 0.5 {
        return 1;
    }
    return -1;
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
