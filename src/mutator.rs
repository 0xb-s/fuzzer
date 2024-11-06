use crate::{
    coverage::CoverageTracker,
    errors::FuzzerError,
    mutator_options::{MutationType, MutatorOptions},
};
use rand::{prelude::StdRng, Rng, SeedableRng};
#[derive(Debug, Clone)]
pub struct Mutator {
    options: MutatorOptions,
    rng: StdRng,
    corpus: Vec<Vec<u8>>,
    coverage_tracker: Option<CoverageTracker>,
}

impl Mutator {
    pub fn new(options: MutatorOptions, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| rand::thread_rng().gen());
        Mutator {
            options,
            rng: StdRng::seed_from_u64(seed),
            corpus: Vec::new(),
            coverage_tracker: None,
        }
    }
    pub fn set_coverage_tracker(&mut self, tracker: CoverageTracker) {
        self.coverage_tracker = Some(tracker);
    }
    pub fn mutate_with_feedback(&mut self, data: &[u8]) -> Result<Vec<u8>, FuzzerError> {
        let mut mutated = data.to_vec();
        let mutations = self.rng.gen_range(1..=self.options.max_mutations);

        for _ in 0..mutations {
            let mutation_type = self.select_mutation_type();
            match mutation_type {
                MutationType::BitFlip => self.bit_flip(&mut mutated),
                MutationType::ByteFlip => self.byte_flip(&mut mutated),
                MutationType::BlockMutation => self.block_mutation(&mut mutated),
                MutationType::Arithmetic => self.arithmetic_mutation(&mut mutated),
                MutationType::Crossover => self.crossover(&mut mutated),
                MutationType::Splicing => self.splicing(&mut mutated),
                MutationType::Replacement => self.replacement(&mut mutated),
                MutationType::Shuffling => self.shuffling(&mut mutated),
                MutationType::InterestingValueInsertion => {
                    self.insert_interesting_value(&mut mutated)
                }
            }
        }

        Ok(mutated)
    }
    fn select_mutation_type(&mut self) -> MutationType {
        let idx = self.rng.gen_range(0..self.options.mutation_types.len());
        self.options.mutation_types[idx].clone()
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
    fn arithmetic_mutation(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_arithmetics || data.is_empty() {
            return;
        }
        let idx = self.rng.gen_range(0..data.len());
        let delta = self
            .rng
            .gen_range(-self.options.arithmetics_range..=self.options.arithmetics_range)
            as i8;
        data[idx] = data[idx].wrapping_add(delta as u8);
    }
    fn block_mutation(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_block_mutation {
            return;
        }
        let size = self.options.block_mutation_size;
        if data.len() < size {
            return;
        }
        let idx = self.rng.gen_range(0..(data.len() - size));
        for i in idx..(idx + size) {
            data[i] = self.rng.gen::<u8>();
        }
    }

    fn crossover(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_crossover || self.corpus.is_empty() {
            return;
        }
        let other = &self.corpus[self.rng.gen_range(0..self.corpus.len())];
        let min_len = data.len().min(other.len());
        let crossover_point = self.rng.gen_range(0..min_len);
        data[crossover_point..].copy_from_slice(&other[crossover_point..]);
    }

    fn splicing(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_splicing || self.corpus.is_empty() {
            return;
        }
        let other = &self.corpus[self.rng.gen_range(0..self.corpus.len())];
        let splice_len = self.rng.gen_range(1..=other.len());
        let splice_point = self.rng.gen_range(0..data.len());
        data.splice(
            splice_point..splice_point,
            other[..splice_len].iter().cloned(),
        );
    }

    fn replacement(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_replacement || self.options.dictionary.is_none() {
            return;
        }
        let dictionary = self.options.dictionary.as_ref().unwrap();
        if dictionary.is_empty() {
            return;
        }
        let word = &dictionary[self.rng.gen_range(0..dictionary.len())];
        let idx = self.rng.gen_range(0..data.len());
        let end = (idx + word.len()).min(data.len());
        data.splice(idx..end, word.iter().cloned());
    }

    fn shuffling(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_shuffling || data.len() < 2 {
            return;
        }
        let idx1 = self.rng.gen_range(0..data.len());
        let idx2 = self.rng.gen_range(0..data.len());
        data.swap(idx1, idx2);
    }

    fn insert_interesting_value(&mut self, data: &mut Vec<u8>) {
        if !self.options.enable_interesting_value_insertion
            || self.options.interesting_values.is_empty()
        {
            return;
        }
        let value = &self.options.interesting_values
            [self.rng.gen_range(0..self.options.interesting_values.len())];
        let idx = self.rng.gen_range(0..=data.len());
        data.splice(idx..idx, value.iter().cloned());
    }

    pub fn add_to_corpus(&mut self, data: Vec<u8>) {
        self.corpus.push(data);
    }
}
