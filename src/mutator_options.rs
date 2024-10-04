#[derive(Debug, Clone)]
pub struct MutatorOptions {
    pub mutation_rate: f64,
    pub max_mutations: usize,
    pub mutation_types: Vec<MutationType>,
    pub enable_crossover: bool,
    pub crossover_rate: f64,
    pub dictionary: Option<Vec<Vec<u8>>>,
    pub preserve_semantics: bool,
    pub enable_splicing: bool,
    pub splicing_rate: f64,
    pub enable_replacement: bool,
    pub replacement_rate: f64,
    pub enable_shuffling: bool,
    pub shuffling_rate: f64,
    pub max_mutation_depth: usize,
    pub enable_arithmetics: bool,
    pub arithmetics_range: i64,
    pub enable_block_mutation: bool,
    pub block_mutation_size: usize,
    pub enable_bit_flip: bool,
    pub bit_flip_probability: f64,
    pub enable_byte_flip: bool,
    pub byte_flip_probability: f64,
    pub enable_interesting_value_insertion: bool,
    pub interesting_values: Vec<Vec<u8>>,
}

impl Default for MutatorOptions {
    fn default() -> Self {
        MutatorOptions {
            mutation_rate: 0.1,
            max_mutations: 5,
            mutation_types: vec![
                MutationType::BitFlip,
                MutationType::ByteFlip,
                MutationType::BlockMutation,
                MutationType::Arithmetic,
                MutationType::Crossover,
                MutationType::Splicing,
                MutationType::Replacement,
                MutationType::Shuffling,
                MutationType::InterestingValueInsertion,
            ],
            enable_crossover: false,
            crossover_rate: 0.05,
            dictionary: None,
            preserve_semantics: false,
            enable_splicing: false,
            splicing_rate: 0.05,
            enable_replacement: false,
            replacement_rate: 0.05,
            enable_shuffling: false,
            shuffling_rate: 0.05,
            max_mutation_depth: 3,
            enable_arithmetics: false,
            arithmetics_range: 10,
            enable_block_mutation: false,
            block_mutation_size: 4,
            enable_bit_flip: true,
            bit_flip_probability: 0.01,
            enable_byte_flip: true,
            byte_flip_probability: 0.01,
            enable_interesting_value_insertion: false,
            interesting_values: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum MutationType {
    BitFlip,
    ByteFlip,
    BlockMutation,
    Arithmetic,
    Crossover,
    Splicing,
    Replacement,
    Shuffling,
    InterestingValueInsertion,
 
}
