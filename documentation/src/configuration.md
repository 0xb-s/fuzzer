# Configuration

Configuring the fuzzer allows you to customize its behavior and optimize it for specific testing needs. This section will explain the options available for setting up the fuzzer, including general configuration options, mutator options, and sanitizers.

To configure the fuzzer, you'll use the `FuzzerConfig` builder pattern. This allows you to chain multiple configuration options and create a customized `FuzzerConfig` object that controls the behavior of the fuzzing process.

## Setting Up the Configuration

Here's a basic example of how to create and apply configuration settings to the fuzzer:

```rust
use fuzzer::FuzzerConfig;
use fuzzer::utils::{FuzzMode, InputFormat};
use std::time::Duration;

let config = FuzzerConfig::builder()
    .input_format(InputFormat::JSON)
    .fuzz_mode(FuzzMode::Mutation)
    .timeout(Duration::from_secs(2))
    .max_iterations(5000)
    .seed(12345)
    .stop_on_first_crash(true)
    .enable_logging(true)
    .save_crashes(true)
    .thread_count(4)
    .build();
```
