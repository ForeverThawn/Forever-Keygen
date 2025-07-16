use std::fmt::Result;

use clap::Parser;

#[macro_use]
mod msg;

mod salt;
mod interactive;
mod keygen;
mod cli;

#[derive(Parser)]
#[command(name = "forever-keygen")]
#[command(about = "Generate encryption keys from 6-digit codes", long_about = None)]
#[command(version)]
struct Args {
    /// 6-digit code for key generation
    #[arg(short, long)]
    code: Option<String>,
    
    /// Salt string for key generation
    #[arg(short, long)]
    salt: Option<String>,

    /// PBKDF2 iterations
    #[arg(short, long, default_value = "100000")]
    iterations: Option<String>,

    /// Output key length in bytes
    #[arg(short, long, default_value = "128")]
    key_length: Option<String>,
}

fn main() -> Result {
    let args = Args::parse();
    
    if args.code.is_none() {
        let interactive_mode = interactive::InteractiveMode::run();
        interactive::InteractiveMode::display_results(&interactive_mode);
        return Ok(())
    }

    cli::run(
        args.code.as_deref().unwrap_or("123456"), 
        args.salt.as_deref().unwrap_or(""),
        args.iterations.as_deref().unwrap_or("100000"),
        args.key_length.as_deref().unwrap_or("128"),
    );

    Ok(())
}


// #[derive(Parser)]
// #[command(name = "forever-keygen")]
// #[command(about = "Generate encryption keys from 6-digit codes")]
// struct Args {
//     /// 6-digit code for key generation
//     #[arg(short, long, default_value = "123456")]
//     code: String,
    
//     /// Salt string for key generation
//     #[arg(short, long)]
//     salt: Option<String>,
// }

// fn main() {
//     let args = Args::parse();
    
//     let salt = generate_salt_from_args(args.salt.as_deref());
//     println!("Generated salt: {}", BASE64.encode(&salt));
    
//     let encryption_key = generate_key(&args.code, &salt);
//     println!("Generated encryption key: {}", encryption_key);
// }
