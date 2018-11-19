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