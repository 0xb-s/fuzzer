use crate::{errors::FuzzerError, mutator_options::MutatorOptions};
use rand::{prelude::StdRng, Rng, SeedableRng};
#[derive(Debug, Clone)]
pub struct Mutator {
    options: MutatorOptions,
    rng: StdRng,
}

impl Mutator {
    pub fn new(options: MutatorOptions, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
        Mutator {
            options,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn mutate(&mut self, data: &[u8]) -> Result<Vec<u8>, FuzzerError> {
        let mut mutated = data.to_vec();
        let mutations = self.rng.gen_range(1..=self.options.max_mutations);

        for _ in 0..mutations {
            let choice = self.rng.gen_range(0..4);
            match choice {
                0 => self.bit_flip(&mut mutated),
                1 => self.byte_flip(&mut mutated),
                2 => self.insert_byte(&mut mutated),
                3 => self.delete_byte(&mut mutated),
                _ => (),
            }
        }
        Ok(mutated)
    }

    fn bit_flip(&mut self, data: &mut Vec<u8>) {
        if data.is_empty() {
            return;
        }
        let idx = self.rng.gen_range(0..data.len());
        let bit = self.rng.gen_range(0..8);
        data[idx] ^= 1 << bit;
    }

    fn byte_flip(&mut self, data: &mut Vec<u8>) {
        if data.is_empty() {
            return;
        }
        let idx = self.rng.gen_range(0..data.len());
        data[idx] = self.rng.gen::<u8>();
    }

    fn insert_byte(&mut self, data: &mut Vec<u8>) {
        let idx = self.rng.gen_range(0..=data.len());
        let byte = self.rng.gen::<u8>();
        data.insert(idx, byte);
    }

    fn delete_byte(&mut self, data: &mut Vec<u8>) {
        if data.is_empty() {
            return;
        }
        let idx = self.rng.gen_range(0..data.len());
        data.remove(idx);
    }
}
