// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!`
/// based argument list.
///
/// # Examples
///
/// ```edition2018
/// use ink_log::{log, Level};
///
/// # fn main() {
/// let data = (42, "Forty-two");
/// let private_data = "private";
///
/// log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
/// log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
///     data.0, data.1, private_data);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $message:expr) => ({
        use ink_log::{LogRecord, FUNC_ID_LOG, Vec, format};
        // ensure that $message is a valid format string literal
        let _ = __log_format_args!($message);
        let input = LogRecord {
            level: $lvl as u32,
            target: Vec::from(format!("{}:{}:{}", $target, __log_file!(), __log_line!())),
            args: Vec::from($message),
        };
        ink_env::call_chain_extension::<LogRecord, LogRecord>(FUNC_ID_LOG, &input)
            .unwrap();
    });
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        use ink_log::{LogRecord, FUNC_ID_LOG, Vec, format};
        let input = LogRecord {
            level: $lvl as u32,
            target: Vec::from(format!("{}:{}:{}", $target, __log_file!(), __log_line!())),
            args: Vec::from(format!("{}", __log_format_args!($($arg)+))),
        };
        ink_env::call_chain_extension::<LogRecord, LogRecord>(FUNC_ID_LOG, &input)
            .unwrap();
    });
    ($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), $lvl, $($arg)+))
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```edition2018
/// use ink_log::error;
///
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// error!(target: "app_events", "App Error: {}, Port: {}", err_info, 22);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Error, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Error, $($arg)+);
    )
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```edition2018
/// use ink_log::warn;
///
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// warn!(target: "input_events", "App received warning: {}", warn_description);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Warn, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Warn, $($arg)+);
    )
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```edition2018
/// use ink_log::info;
///
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(target: "connection_events", "Successfull connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Info, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Info, $($arg)+);
    )
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```edition2018
/// use ink_log::debug;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Debug, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Debug, $($arg)+);
    )
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```edition2018
/// use ink_log::trace;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(target: "app_events", "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => (
        log!(target: $target, $crate::Level::Trace, $($arg)+);
    );
    ($($arg:tt)+) => (
        log!($crate::Level::Trace, $($arg)+);
    )
}

// The log macro above cannot invoke format_args directly because it uses
// local_inner_macros. A format_args invocation there would resolve to
// $crate::format_args which does not exist. Instead invoke format_args here
// outside of local_inner_macros so that it resolves (probably) to
// core::format_args or std::format_args. Same for the several macros that
// follow.
//
// This is a workaround until we drop support for pre-1.30 compilers. At that
// point we can remove use of local_inner_macros, use $crate:: when invoking
// local macros, and invoke format_args directly.
#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}
