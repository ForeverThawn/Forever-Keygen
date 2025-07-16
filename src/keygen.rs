use data_encoding::BASE64;
use ring::{pbkdf2};

use std::num::NonZeroU32;

pub const ITERATIONS: u32 = 100_000;   // PBKDF2 迭代次数
pub const KEY_LENGTH: usize = 128;     // 输出密钥长度 128Bytes

pub fn generate_key(code: &str, salt: &[u8], iterations: u32, key_length: usize) -> String {
    let mut output = vec![0u8; key_length];
    
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        salt,
        code.as_bytes(),
        &mut output
    );
    
    BASE64.encode(&output)
}
