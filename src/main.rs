use crypto_hash::{Algorithm, hex_digest};

struct Block {
    value: &'static str,
    hash: String,
    nonce: u32
}

impl Block {
    fn new(value: &'static str) -> Block {
        let mut block = Block {
            value,
            hash: String::new(),
            nonce: 0
        };
        block.generate_hash();
        block
    }

    fn generate_hash(&mut self) {
        let hash_data = self.nonce.to_string() + self.value;
        self.hash= hex_digest(Algorithm::SHA256, hash_data.as_bytes());
    }

    fn get_value(&self) -> &'static str {
        self.value
    }

    fn get_hash(&self) -> &str {
        &self.hash
    }

    fn mine(&mut self, difficulty: usize) {
        let zeros = str::repeat("0", difficulty);

        let get_hash_slice= |hash: &str, size: usize| -> String {
            hash[0..size].to_owned()
        };

        while get_hash_slice(&self.hash, difficulty) != zeros {
            self.generate_hash();
            self.nonce = self.nonce + 1;
        }
        println!("Block successfully mined");
    }
}

fn main() {
    let mut block = Block::new("Block 1");
    println!("VALUE: {:?}", block.get_value());
    block.mine(4);
    println!("HASH: {:?}", block.get_hash());
}

#[cfg(test)]
mod tests {
    use crate::Block;
        
    #[test]
    fn block_instantiated_with_correct_properties() {
        let name = "Test Block 1";
        let block = Block::new(name);
        assert_eq!(block.get_value(), name);
        assert_eq!(block.get_hash().len(), 64);
        assert_eq!(block.nonce, 0);
    }

    #[test]
    fn block_is_mined() {
        let mut block = Block::new("Test Block 2");
        let difficulty = 3;
        block.mine(difficulty);
        let hash = block.get_hash();
        assert_eq!(&hash[..difficulty], str::repeat("0", difficulty));
        assert_eq!(hash.len(), 64);
    }
}