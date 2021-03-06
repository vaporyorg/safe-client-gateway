macro_rules! concat_parts {
    ($parts_head:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!(
            "{}{}",
            $crate::models::service::transactions::ID_SEPARATOR,
            $parts_head
        );
    };
    ($parts_head:expr, $($parts_tail:expr),+) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!(
            "{}{}{}",
            $crate::models::service::transactions::ID_SEPARATOR,
            $parts_head,
            concat_parts!($($parts_tail),+)
        );
    };
}

macro_rules! create_id {
    ($tx_type:expr, $($parts:expr),+) => {
        // `stringify!` will convert the expression *as it is* into a string.
        format!("{}{}", $tx_type, concat_parts!($($parts),+));
    };
}

macro_rules! bail {
    ($msg:literal $(,)?) => {
        return Err($crate::api_error!($msg))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::api_error!($fmt, $($arg)*))
    };
}

#[macro_export]
macro_rules! api_error {
    ($msg:literal $(,)?) => {
        // Handle $:literal as a special case to make cargo-expanded code more
        // concise in the common case.
        $crate::utils::errors::ApiError::new_from_message($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::utils::errors::ApiError::new_from_message(format!($fmt, $($arg)*))
        $crate::private::new_adhoc()
    };
}

#[macro_export]
macro_rules! client_error {
    ($status_code:expr, $message:expr) => {
        $crate::utils::errors::ApiError::new_from_message_with_code($status_code, format!($message))
    };
}

macro_rules! to_hex_string {
    ($input:expr) => {{
        let mut output = String::new();
        for byte in $input.iter() {
            output.push_str(&format!("{:02x?}", byte)) // uppercase x is for uppercase hex char.
        }
        format!("0x{}", output)
    }};
}
