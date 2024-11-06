# Mutator Options

The `MutatorOptions` structure allows you to control how inputs are mutated during the fuzzing process. These options help define the types and frequency of mutations that the fuzzer will apply, enabling you to customize the way inputs evolve and explore potential vulnerabilities in the target function.

## Setting Up Mutator Options

To configure mutation behaviors, create a `MutatorOptions` instance and pass it to your fuzzer configuration:

```rust
use fuzzer::mutator_options::{MutatorOptions, MutationType};

let mutator_options = MutatorOptions {
    mutation_rate: 0.2,
    max_mutations: 10,
    mutation_types: vec![MutationType::BitFlip, MutationType::ByteFlip],
    enable_crossover: true,
    crossover_rate: 0.05,
    block_mutation_size: 16,
    arithmetics_range: 10,
    interesting_values: vec![vec![0x00], vec![0xFF], vec![0x7F]],
    ..Default::default()
};
``` 

After defining the options, pass mutator_options into the FuzzerConfig:

```rust
let config = FuzzerConfig::builder()
    .mutator_options(mutator_options)
    .build();
```

