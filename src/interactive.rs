use std::io;
// use dialoguer::{Input, Password};
use dialoguer::{Input, Confirm};
use data_encoding::BASE64;
use crate::{keygen, salt};

pub struct InteractiveMode {
    code: String,
    use_random_salt: bool,
    salt: Vec<u8>,
    encryption_key: String,
}

impl InteractiveMode {
    pub fn run() -> Self {
        println!("Forever Key Generator - Interactive Mode");
        println!("---------------------------------------");
        
        let code = Self::prompt_code();
        let use_random_salt = Self::prompt_salt_option();
        let salt = Self::prompt_salt_value(use_random_salt);
        let encryption_key = keygen::generate_key(&code, &salt, keygen::ITERATIONS, keygen::KEY_LENGTH);

        Self {
            code,
            use_random_salt,
            salt,
            encryption_key,
        }
    }

    pub fn display_results(&self) {
        println!("\nKey Generation Complete!");
        println!("-----------------------");
        msg_yellow!("6-digit code", "{}", self.code);
        msg_yellow!("PBKDF2 iterations", "100000");
        msg_yellow!("Key length", "128");
        println!("If you want to change these values, please use the CLI options.");
        
        let salt_display = BASE64.encode(&self.salt);
        if self.use_random_salt {
            msg_cyan!("Salt (random)", "{}", salt_display);
        } else {
            msg_cyan!("Salt", "{}", salt_display);
        }
        
        msg_green!("Encryption key", "{}", self.encryption_key);
        pause();
    }

    fn prompt_code() -> String {
        Input::new()
            .with_prompt("Enter 6-digit code")
            .validate_with(|input: &String| {
                if input.len() != 6 || !input.chars().all(|c| c.is_ascii_digit()) {
                    msg_error!("Invalid code.");
                    Err("Code must be exactly 6 digits.")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap()
    }

    fn prompt_salt_option() -> bool {
        Confirm::new()
            .with_prompt("Generate a random salt?")
            .default(false)
            .interact()
            .unwrap()
    }

    fn prompt_salt_value(use_random_salt: bool) -> Vec<u8> {
        if use_random_salt {
            salt::generate_random_salt()
        } else {
            let salt_str: String = Input::new()
                .with_prompt("Enter salt string")
                .interact_text()
                .unwrap();
            salt::generate_salt_from_args(Some(&salt_str))
        }
    }

}

pub fn pause() {
    // let mut stdout = io::stdout();
    let stdin = io::stdin();

    // write!(stdout, "Press Enter to continue...").expect("Failed to write to stdout");
    // stdout.flush().expect("Failed to flush stdout");
    
    println!("Press Enter to continue...");
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
}