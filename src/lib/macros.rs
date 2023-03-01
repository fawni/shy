#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!(
            "{}{}{} {}",
            "[".bright_black(),
            "+".green(),
            "]".bright_black(),
            format!($($arg)*))
    };
}
