use fuzzer::mutator_options::{MutationType, MutatorOptions};
use fuzzer::utils::{FuzzMode, InputFormat};
use fuzzer::{Fuzzer, FuzzerConfig, FuzzerError, TargetFunction};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mutator_options = MutatorOptions {
        mutation_rate: 0.1,
        max_mutations: 5,
        mutation_types: vec![
            MutationType::BitFlip,
            MutationType::ByteFlip,
            MutationType::BlockMutation,
        ],

        ..Default::default()
    };

    let config = FuzzerConfig::builder()
        .input_format(InputFormat::Text)
        .fuzz_mode(FuzzMode::Mutation)
        .timeout(Duration::from_secs(1))
        .max_iterations(1000)
        .seed(42)
        .mutator_options(mutator_options)
        .stop_on_first_crash(false)
        .stats_interval(100)
        .max_input_size(256)
        .min_input_size(1)
        .build();

    let mut fuzzer = Fuzzer::new(config);

    let from_utf8_target =
        |input: &[u8]| -> Pin<Box<dyn Future<Output = Result<(), FuzzerError>> + Send>> {
            let input_owned = input.to_owned();

            Box::pin(async move {
                match std::str::from_utf8(&input_owned) {
                    Ok(valid_str) => {
                        println!("Valid UTF-8 string: {}", valid_str);
                        Ok(())
                    }
                    Err(e) => Err(FuzzerError::ExecutionError(format!(
                        "from_utf8 error: {}",
                        e
                    ))),
                }
            })
        };

    let target = TargetFunction::new_async("FromUtf8", from_utf8_target);

    fuzzer.add_target(target);

    if let Err(e) = fuzzer.run().await {
        eprintln!("Fuzzer encountered an error: {}", e);
    }
}
