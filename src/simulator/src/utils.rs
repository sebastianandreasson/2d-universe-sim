use cfg_if::cfg_if;

pub fn rand_int(n: i32) -> i32 {
    (js_sys::Math::random() * n as f64) as i32
}

pub fn rand_dir() -> i32 {
    let i = rand_int(1000);
    (i % 3) - 1
}

pub fn rand_dir_2() -> i32 {
    let i = rand_int(1000);
    if (i % 2) == 0 {
        -1
    } else {
        1
    }
}

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
         pub fn set_panic_hook() {}
    }
}
