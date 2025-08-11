use sha2::{Digest, Sha256};

pub fn hash_now(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:X}", hasher.finalize()).to_string()
}