#[macro_export]
macro_rules! log {
    (source: $source:expr, $lvl:expr, $($key:tt = $value:expr),+; $msg:expr) => ({
        let lvl = $lvl;
        if $crate::models::__private_api::log_enabled(lvl, $source) {

            $crate::models::__private_api::log(
                $msg.to_string(),
                lvl,
                &($source, models::__private_api::module_path!(), $crate::models::__private_api::file!()),
                $crate::models::__private_api::line!(),
                $crate::models::__private_api::Option::Some(&[$(models::__log_key!($key, $value)),+]),
            );
        }
    });

    (source: $source:expr, $lvl:expr, $msg:expr) => ({
        let lvl = $lvl;
        if $crate::models::__private_api::log_enabled(lvl, $source) {
            $crate::models::__private_api::log(
                $msg.to_string(),
                lvl,
                &($source, $crate::models::__private_api::module_path!(), $crate::models::__private_api::file!()),
                $crate::models::__private_api::line!(),
                $crate::models::__private_api::Option::None,
            );
        }
    });

    ($lvl:expr, $msg:expr) => ($crate::log!(source: $crate::models::__private_api::module_path!(), $lvl, $msg));
}

#[macro_export]
macro_rules! log_console {

    ($source:expr, $lvl:expr, $thread:expr, $msg:expr) => ({
        let lvl = $lvl;
        if $crate::models::__private_api::log_enabled(lvl, $source) {
            $crate::models::__private_api::log_console(
                $msg.to_string(),
                lvl,
                $source.to_string(),
                $thread,
                None
            );
        }
    });
}

#[macro_export]
macro_rules! log_enabled {
    (source: $source:expr, $lvl:expr) => {
        $crate::models::__private_api::log_enabled($lvl, $source)
    };
    ($lvl:expr) => {
        $crate::models::__private_api::log_enabled($lvl, $crate::models::__private_api::module_path!())
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_key {
    ($key:ident, $value:expr) => {
        ($crate::models::__private_api::stringify!($key), &$value)
    };
    ($key:expr, $value:expr) => {
        ($key, &$value)
    };
}
