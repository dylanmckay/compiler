#[macro_export]
macro_rules! debug_log {
    ($system:expr, $message:expr) => {
        println!("[{}]: {}", $system, $message);
    }
}

