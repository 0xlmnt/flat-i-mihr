use ring::digest::SHA512_OUTPUT_LEN;
use argon2::{Config, ThreadMode, Variant, Version};
use ring::rand::{SystemRandom, SecureRandom};

pub fn hash(password: String, salt: &[u8]) -> String {
    let config = Config {
        ad: &[],
        hash_length: 32,
        lanes: 4,
        mem_cost: 65536,
        secret: &[],
        thread_mode: ThreadMode::Parallel,
        time_cost: 10,
        variant: Variant::Argon2id,
        version: Version::Version13
    };

    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}

pub fn generate_salt() -> Vec<u8> {
    let rng = SystemRandom::new();
    let mut salt = [0u8; SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).expect("Error on creating salt.");
    salt.to_vec()
}

pub fn salt_to_string(salt: &[u8]) -> String {
    String::from(std::str::from_utf8(salt).unwrap())
}
