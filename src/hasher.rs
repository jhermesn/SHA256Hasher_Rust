use sha2::{Digest, Sha256};
pub struct Sha256Hasher;

impl Sha256Hasher {
    pub fn hash(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn verify(input: &str, expected_hash: &str) -> bool {
        Self::hash(input).eq_ignore_ascii_case(expected_hash)
    }
}
