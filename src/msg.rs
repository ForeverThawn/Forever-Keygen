/// 提供固定格式消息到stdout的输出
/// "{prefix}: {msg}" 
/// 

#[macro_export]
macro_rules! msg_error {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("[{}] {}", "Error".red().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! msg_red {
    ($prefix:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        println!("{}: {}", $prefix.red().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! msg_green {
    ($prefix:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        println!("{}: {}", $prefix.green().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! msg_cyan {
    ($prefix:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        println!("{}: {}", $prefix.cyan().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! msg_yellow {
    ($prefix:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        println!("{}: {}", $prefix.yellow().bold(), format_args!($($arg)*));
    }};
}

