use crate::analysis::Analyzer;
use crate::errors::FuzzerError;
use crate::input::InputGenerator;
use crate::mutator::Mutator;
use crate::target::Executable;
use crate::target::TargetFunction;
use crate::utils::{ExecutionResult, FuzzMode};
use crate::FuzzerConfig;

use log::{error, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::task;
use tokio::time::{timeout, Duration};
#[derive(Clone)]
pub struct Fuzzer {
    config: Arc<FuzzerConfig>,
    pub input_generator: InputGenerator,
    pub mutator: Mutator,
    analyzer: Analyzer,
    targets: Vec<TargetFunction>,
    stats: Arc<Mutex<FuzzerStats>>,
    start_time: Instant,
}

#[derive(Default, Clone, Debug)]
pub struct FuzzerStats {
    pub total_runs: usize,
    pub successful_runs: usize,
    pub errors: usize,
    pub timeouts: usize,
    pub unique_crashes: HashMap<String, usize>,
    pub total_crashes: usize,
    pub total_time: Duration,
    pub inputs_tested: usize,
    // Additional statistics can be added here
}

impl Fuzzer {
    pub fn new(config: FuzzerConfig) -> Self {
        if config.enable_logging {
            if let Some(ref log_file) = config.log_file {
                let _ = log4rs::init_file(log_file, Default::default());
            } else {
                let _ = env_logger::builder().is_test(true).try_init();
            }
        }

        let input_generator = InputGenerator::new(config.clone());
        let mutator = Mutator::new(config.mutator_options.clone(), config.seed);
        Fuzzer {
            config: Arc::new(config),
            input_generator,
            mutator,
            analyzer: Analyzer::new(),
            targets: Vec::new(),
            stats: Arc::new(Mutex::new(FuzzerStats::default())),
            start_time: Instant::now(),
        }
    }

    pub fn add_target(&mut self, target: TargetFunction) {
        self.targets.push(target);
    }

    pub async fn run(&mut self) -> Result<(), FuzzerError> {
        let mut iteration = 0;
        let max_iterations = self.config.max_iterations;
        let stop_time = self.config.max_total_time.map(|t| self.start_time + t);

        while iteration < max_iterations {
            if let Some(stop_time) = stop_time {
                if Instant::now() >= stop_time {
                    info!("Maximum total time reached. Stopping fuzzing.");
                    break;
                }
            }

            let mut x = self.input_generator.clone();
            let inp = x.generate_input();
            let mut input = match inp {
                Ok(input) => input,
                Err(e) => {
                    error!("Input generation error: {}", e);
                    continue;
                }
            };

            if matches!(self.config.fuzz_mode, FuzzMode::Mutation | FuzzMode::Hybrid) {
                let mut mutotator = self.mutator.clone();
                match mutotator.mutate(&input) {
                    Ok(mutated_input) => input = mutated_input,
                    Err(e) => {
                        error!("Mutation error: {}", e);
                        continue;
                    }
                }
            }

            self.stats.lock().unwrap().inputs_tested += 1;

            let stats = Arc::clone(&self.stats);
            let config = Arc::clone(&self.config);
            let cloned_input = input.clone();
            let targets = self.targets.clone();

            let tasks: Vec<_> = targets
                .iter()
                .map(|target| {
                    let timeout_duration = self.config.timeout;
                    let c = cloned_input.clone();
                    let target_clone = target.clone();
                    let target_name = target.name.clone();
                    let exec_future = Self::execute_target(target_clone, c, timeout_duration);

                    task::spawn({
                        let stats = Arc::clone(&stats);
                        let config = Arc::clone(&config);
                        let cloned_input = cloned_input.clone();

                        async move {
                            let exec_result = exec_future.await;
                            Fuzzer::update_stats(&stats, &exec_result);

                            if config.enable_logging {
                                info!("Target: {}, Result: {:?}", target_name, exec_result);
                            }

                            if config.save_crashes {
                                if let ExecutionResult::Crash(ref crash_info) = exec_result {
                                    Fuzzer::save_crash(&config, &cloned_input, crash_info);
                                }
                            }
                        }
                    })
                })
                .collect();

            futures::future::join_all(tasks).await;

            iteration += 1;

            if iteration % self.config.stats_interval as u64 == 0 {
                self.print_stats(iteration as usize);
            }

            if self.config.stop_on_first_crash && self.stats.lock().unwrap().total_crashes > 0 {
                info!("Crash detected. Stopping fuzzing.");
                break;
            }
        }

        self.analyzer.report();
        Ok(())
    }

    async fn execute_target(
        target: TargetFunction,
        input: Vec<u8>,
        timeout_duration: Duration,
    ) -> ExecutionResult {
        let exec = target.execute(&input);

        match timeout(timeout_duration, exec).await {
            Ok(Ok(_)) => ExecutionResult::Success,
            Ok(Err(e)) => ExecutionResult::Crash(e.to_string()),
            Err(_) => ExecutionResult::Timeout,
        }
    }

    fn update_stats(stats: &Arc<Mutex<FuzzerStats>>, result: &ExecutionResult) {
        let mut stats = stats.lock().unwrap();
        stats.total_runs += 1;
        match result {
            ExecutionResult::Success => stats.successful_runs += 1,
            ExecutionResult::Crash(crash_info) => {
                stats.errors += 1;
                stats.total_crashes += 1;
                *stats.unique_crashes.entry(crash_info.clone()).or_insert(0) += 1;
            }
            ExecutionResult::Timeout => stats.timeouts += 1,
            _ => {}
        }
    }

    fn print_stats(&self, iteration: usize) {
        let stats = self.stats.lock().unwrap();
        let elapsed = Instant::now() - self.start_time;
        println!("=== Fuzzing Iteration: {} ===", iteration);
        println!("Total runs: {}", stats.total_runs);
        println!("Successful runs: {}", stats.successful_runs);
        println!("Errors (Crashes): {}", stats.errors);
        println!("Timeouts: {}", stats.timeouts);
        println!("Unique crashes: {}", stats.unique_crashes.len());
        println!("Total crashes: {}", stats.total_crashes);
        println!("Inputs tested: {}", stats.inputs_tested);
        println!("Elapsed time: {:?}", elapsed);
        // Additional statistics can be printed here
        println!("==============================");
    }

    fn save_crash(config: &FuzzerConfig, input: &Vec<u8>, crash_info: &str) {
        if let Some(ref dir) = config.crash_directory {
            use std::fs::{self, OpenOptions};
            use std::io::Write;
            let _ = fs::create_dir_all(dir);
            let filename = format!("{}/crash_{}.bin", dir, uuid::Uuid::new_v4());
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&filename)
            {
                let _ = file.write_all(input);
                let _ = file.write_all(b"\n");
                let _ = file.write_all(crash_info.as_bytes());
            }
        }
    }
}
