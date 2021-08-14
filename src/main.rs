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
        let nonce_string = self.nonce.to_string();
        let hash_string = nonce_string + self.value;
        let hash = hex_digest(Algorithm::SHA256, hash_string.as_bytes());
        self.hash = hash;
    }

    fn get_value(&self) -> &'static str {
        self.value
    }

    fn get_hash(&self) -> &str {
        &self.hash
    }

    fn mine(&mut self, difficulty: usize) {
        let mut zeros: String = String::new();
        for _i in 0..difficulty {
            zeros.push('0');
        }

        let get_hash_slice= |hash: &str, size: usize| -> String {
            let slice = &hash[0..size];
            slice.to_owned()
        };

        while get_hash_slice(&self.hash, difficulty) != zeros.as_str() {
            self.generate_hash();
            self.nonce = self.nonce + 1;
        }
        println!("Block successfully mined!");
    }
}

fn main() {
    let mut block: Block = Block::new("block 1");
    println!("VALUE: {:?}", block.get_value());
    block.mine(6);
    println!("HASH: {:?}", block.get_hash());
    block.mine(6);
    println!("HASH: {:?}", block.get_hash());
}