use ring::rand;


pub fn generate_random_salt() -> Vec<u8> {
    let rng = rand::SystemRandom::new();
    let salt: [u8; 16] = rand::generate(&rng).unwrap().expose();  // 128位随机盐
    salt.to_vec()
}

pub fn generate_salt_from_args(salt_str: Option<&str>) -> Vec<u8> {
    if let Some(salt) = salt_str {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        salt.hash(&mut hasher);
        let hash = hasher.finish();
        
        // hash扩展为16字节
        let mut salt_bytes = [0u8; 16];
        salt_bytes[..8].copy_from_slice(&hash.to_le_bytes());
        salt_bytes[8..].copy_from_slice(&hash.to_be_bytes());
        salt_bytes.to_vec()
    } else {

        generate_random_salt()
    }
}