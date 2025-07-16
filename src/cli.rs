use crate::salt;
use crate::keygen;
use std::process;
use data_encoding::BASE64;

fn parse_iterations(iterations: &str) -> u32 {
    iterations.parse().unwrap_or_else(|_| {
        msg_error!("Invalid iterations value: {}", iterations);
        process::exit(1);
    })
}

fn parse_key_length(key_length: &str) -> usize {
    key_length.parse().unwrap_or_else(|_| {
        msg_error!("Invalid key length value: {}", key_length);
        process::exit(1);
    })
}

pub fn run(code: &str, salt_str: &str, iterations: &str, key_length: &str) ->Option<()> {
    let iterations_val = parse_iterations(iterations);
    let key_length_val = parse_key_length(key_length);

    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        msg_error!("Code must be exactly 6 code");
        process::exit(1);
    }

    let salt = if salt_str.is_empty() {
        salt::generate_random_salt()
    } else {
        salt::generate_salt_from_args(Some(salt_str))
    };

    let encryption_key = keygen::generate_key(code, &salt, iterations_val, key_length_val);

    
    msg_cyan!("Salt", "{}", BASE64.encode(&salt));
    msg_green!("Encryption key", "{}", encryption_key);

    Some(())
}
