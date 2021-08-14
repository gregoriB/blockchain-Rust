use crypto_hash::{Algorithm, hex_digest};

pub trait Hashing {
    fn get_hashing_strings(&self) -> Vec<String>;

    fn generate_hash(&mut self) -> String {
        let strings = self.get_hashing_strings();
        let mut hash_data = String::new();
        for string  in &strings {
            hash_data.push_str(&string);
        }
        hex_digest(Algorithm::SHA256, hash_data.as_bytes())
    }
}
