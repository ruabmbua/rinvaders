//! Easy logging module. Provides utilities for logging on the web platform.

/// Log macro. Takes a format string like the one taken by the *rust formatter API*, 
/// and a variadic number of arguments, which are interpolated into the resulting log
/// message string.
/// 
/// An invocation allocates a new string buffer per call, so it has some performance
/// impact.
/// 
/// TODO: Because wasm is inherently single threaded (for now), consider creating a
/// global formatting buffer, which is reused for logging calls.
#[macro_export]
macro_rules! log {
    ($fmt:expr, $( $arg:expr ),*) => {
        use web_sys::console;
        let s = format!($fmt, 
            $( $arg ),*
        );
        console::log_1(&s.into());
    };
}