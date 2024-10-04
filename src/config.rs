use crate::{
    mutator_options::MutatorOptions,
    utils::{FuzzMode, InputFormat},
};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FuzzerConfig {
    pub input_format: InputFormat,
    pub fuzz_mode: FuzzMode,
    pub timeout: Duration,
    pub max_iterations: u64,
    pub seed: Option<u64>,
    pub mutator_options: MutatorOptions,
    pub stop_on_first_crash: bool,
    pub stats_interval: usize,
    pub max_input_size: usize,
    pub min_input_size: usize,
    pub enable_logging: bool,
    pub log_file: Option<String>,
    pub save_crashes: bool,
    pub crash_directory: Option<String>,
    pub thread_count: usize,
    pub corpus_directory: Option<String>,
    pub dictionary_file: Option<String>,
    pub max_total_time: Option<Duration>,
    pub coverage_enabled: bool,
    pub coverage_directory: Option<String>,
    pub retry_on_timeout: bool,
    pub max_retries: usize,
    pub initial_inputs: Vec<Vec<u8>>,
    pub use_corpus: bool,
    pub corpus_sampling_rate: f64,
    pub sanitizer_enabled: bool,
    pub sanitizer_options: SanitizerOptions,
}

#[derive(Debug, Clone)]
pub struct SanitizerOptions {
    pub address: bool,
    pub thread: bool,
    pub memory: bool,
    pub undefined_behavior: bool,
    pub leak: bool,
}

impl FuzzerConfig {
    pub fn builder() -> FuzzerConfigBuilder {
        FuzzerConfigBuilder::new()
    }
}

pub struct FuzzerConfigBuilder {
    config: FuzzerConfig,
}

impl FuzzerConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: FuzzerConfig {
                input_format: InputFormat::Binary,
                fuzz_mode: FuzzMode::Random,
                timeout: Duration::from_secs(1),
                max_iterations: 1000,
                seed: None,
                mutator_options: MutatorOptions::default(),
                stop_on_first_crash: false,
                stats_interval: 100,
                max_input_size: 1024,
                min_input_size: 1,
                enable_logging: false,
                log_file: None,
                save_crashes: false,
                crash_directory: None,
                thread_count: 1,
                corpus_directory: None,
                dictionary_file: None,
                max_total_time: None,
                coverage_enabled: false,
                coverage_directory: None,
                retry_on_timeout: false,
                max_retries: 3,
                initial_inputs: vec![],
                use_corpus: false,
                corpus_sampling_rate: 0.1,
                sanitizer_enabled: false,
                sanitizer_options: SanitizerOptions {
                    address: false,
                    thread: false,
                    memory: false,
                    undefined_behavior: false,
                    leak: false,
                },
            },
        }
    }

    pub fn input_format(mut self, format: InputFormat) -> Self {
        self.config.input_format = format;
        self
    }

    pub fn fuzz_mode(mut self, mode: FuzzMode) -> Self {
        self.config.fuzz_mode = mode;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn max_iterations(mut self, iterations: u64) -> Self {
        self.config.max_iterations = iterations;
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    pub fn mutator_options(mut self, options: MutatorOptions) -> Self {
        self.config.mutator_options = options;
        self
    }

    pub fn stop_on_first_crash(mut self, stop: bool) -> Self {
        self.config.stop_on_first_crash = stop;
        self
    }

    pub fn stats_interval(mut self, interval: usize) -> Self {
        self.config.stats_interval = interval;
        self
    }

    pub fn max_input_size(mut self, size: usize) -> Self {
        self.config.max_input_size = size;
        self
    }

    pub fn min_input_size(mut self, size: usize) -> Self {
        self.config.min_input_size = size;
        self
    }

    pub fn enable_logging(mut self, enable: bool) -> Self {
        self.config.enable_logging = enable;
        self
    }

    pub fn log_file(mut self, file: String) -> Self {
        self.config.log_file = Some(file);
        self
    }

    pub fn save_crashes(mut self, save: bool) -> Self {
        self.config.save_crashes = save;
        self
    }

    pub fn crash_directory(mut self, directory: String) -> Self {
        self.config.crash_directory = Some(directory);
        self
    }

    pub fn thread_count(mut self, count: usize) -> Self {
        self.config.thread_count = count;
        self
    }

    pub fn corpus_directory(mut self, directory: String) -> Self {
        self.config.corpus_directory = Some(directory);
        self
    }

    pub fn dictionary_file(mut self, file: String) -> Self {
        self.config.dictionary_file = Some(file);
        self
    }

    pub fn max_total_time(mut self, time: Duration) -> Self {
        self.config.max_total_time = Some(time);
        self
    }

    pub fn coverage_enabled(mut self, enabled: bool) -> Self {
        self.config.coverage_enabled = enabled;
        self
    }

    pub fn coverage_directory(mut self, directory: String) -> Self {
        self.config.coverage_directory = Some(directory);
        self
    }

    pub fn retry_on_timeout(mut self, retry: bool) -> Self {
        self.config.retry_on_timeout = retry;
        self
    }

    pub fn max_retries(mut self, retries: usize) -> Self {
        self.config.max_retries = retries;
        self
    }

    pub fn initial_inputs(mut self, inputs: Vec<Vec<u8>>) -> Self {
        self.config.initial_inputs = inputs;
        self
    }

    pub fn use_corpus(mut self, use_corpus: bool) -> Self {
        self.config.use_corpus = use_corpus;
        self
    }

    pub fn corpus_sampling_rate(mut self, rate: f64) -> Self {
        self.config.corpus_sampling_rate = rate;
        self
    }

    pub fn sanitizer_enabled(mut self, enabled: bool) -> Self {
        self.config.sanitizer_enabled = enabled;
        self
    }

    pub fn sanitizer_options(mut self, options: SanitizerOptions) -> Self {
        self.config.sanitizer_options = options;
        self
    }

    pub fn build(self) -> FuzzerConfig {
        self.config
    }
}
