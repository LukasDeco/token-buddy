extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha384;
use rand::{thread_rng, Rng};

pub struct TokenGenerator {
    rng: rand::rngs::ThreadRng,
    sha: Sha384,
}

impl TokenGenerator {
    pub fn new() -> TokenGenerator {
        TokenGenerator {
            rng: thread_rng(),
            sha: Sha384::new(),
        }
    }

    pub fn generate(&mut self, length: usize) -> String {
        let mut token = vec![0u8; length];
        self.rng.fill(&mut token[..]);
        self.sha.input(&token);
        let mut output = [0; 48];
        self.sha.result(&mut output);
        self.sha.reset();
        hex::encode(output)
    }
}
