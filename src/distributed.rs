use crate::config::FuzzerConfig;
use crate::coverage::{CoverageData, CoverageTracker};
use crate::errors::FuzzerError;

use crate::fuzz_engine::FuzzerStats;

use crate::utils::ExecutionResult;
use crate::Fuzzer;
use serde::{Deserialize, Serialize};
use serde_json::{self};

use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task;

#[derive(Clone)]
pub struct DistributedFuzzer {
    config: Arc<FuzzerConfig>,
    fuzzer: Arc<Fuzzer>,
    workers: Vec<WorkerInfo>,
    coverage_tracker: CoverageTracker,
    stats: Arc<Mutex<FuzzerStats>>,
}

#[derive(Clone)]
pub struct WorkerInfo {
    address: String,
    _status: WorkerStatus,
}

#[derive(Clone, Debug)]
pub enum WorkerStatus {
    Idle,
    Busy,
    Offline,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DistributedMessage {
    Task(FuzzTask),
    Result(FuzzResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuzzTask {
    pub input: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuzzResult {
    pub input: Vec<u8>,
    pub result: ExecutionResult,
    pub coverage: Option<CoverageData>,
}

impl DistributedFuzzer {
    pub fn new(config: FuzzerConfig, fuzzer: Fuzzer) -> Self {
        DistributedFuzzer {
            config: Arc::new(config),
            fuzzer: Arc::new(fuzzer),
            workers: Vec::new(),
            coverage_tracker: CoverageTracker::new(),
            stats: Arc::new(Mutex::new(FuzzerStats::default())),
        }
    }

    pub async fn add_worker(&mut self, address: &str) {
        let worker = WorkerInfo {
            address: address.to_string(),
            _status: WorkerStatus::Idle,
        };
        self.workers.push(worker);
    }

    pub async fn run(&mut self) -> Result<(), FuzzerError> {
        let (tx, mut rx): (Sender<FuzzResult>, Receiver<FuzzResult>) = mpsc::channel(100);

     
        for worker in self.workers.clone() {
            let tx_clone = tx.clone();
            let config = Arc::clone(&self.config);
            let fuzzer = Arc::clone(&self.fuzzer);
            let coverage_tracker = self.coverage_tracker.clone();
            let stats = Arc::clone(&self.stats);

            task::spawn(async move {
                if let Err(e) = DistributedFuzzer::worker_loop(
                    worker,
                    config,
                    fuzzer,
                    coverage_tracker,
                    tx_clone,
                    stats,
                )
                .await
                {
                    eprintln!("Worker error: {}", e);
                }
            });
        }

        drop(tx); 

     
        while let Some(fuzz_result) = rx.recv().await {
            self.update_stats_with_result(&fuzz_result);
        }

        Ok(())
    }

    async fn worker_loop(
        worker: WorkerInfo,
        config: Arc<FuzzerConfig>,
        fuzzer: Arc<Fuzzer>,
        coverage_tracker: CoverageTracker,
        tx: Sender<FuzzResult>,
        stats: Arc<Mutex<FuzzerStats>>,
    ) -> Result<(), FuzzerError> {
        let mut stream = TcpStream::connect(&worker.address).await.unwrap();
        loop {
            let mut input_generator = fuzzer.input_generator.clone();
            let input = match input_generator.generate_input() {
                Ok(input) => input,
                Err(e) => {
                    eprintln!("Input generation error: {}", e);
                    continue;
                }
            };

      
            let mut mutated_input = input.clone();
            if matches!(
                config.fuzz_mode,
                crate::utils::FuzzMode::Mutation | crate::utils::FuzzMode::Hybrid
            ) {
                let mut mutator = fuzzer.mutator.clone();
                match mutator.mutate(&input) {
                    Ok(mutated) => mutated_input = mutated,
                    Err(e) => {
                        eprintln!("Mutation error: {}", e);
                        continue;
                    }
                }
            }

            let task = FuzzTask {
                input: mutated_input.clone(),
            };
            let task_json = serde_json::to_string(&DistributedMessage::Task(task)).unwrap();
            stream.write_all(task_json.as_bytes()).await.unwrap();

         
            let mut buffer = vec![0; 4096];
            let n = stream.read(&mut buffer).await.unwrap();
            if n == 0 {
                break;
            }
            let response: DistributedMessage = serde_json::from_slice(&buffer[..n]).unwrap();

            if let DistributedMessage::Result(result) = response {
                DistributedFuzzer::update_stats(&stats, &result);
                if let Some(coverage) = &result.coverage {
                    coverage_tracker.merge(&CoverageTracker {
                        data: Arc::new(Mutex::new(coverage.clone())),
                    });
                }
         
                tx.send(result).await.unwrap();
            }
        }
        Ok(())
    }

    fn update_stats_with_result(&self, result: &FuzzResult) {
        let mut stats = self.stats.lock().unwrap();
        Self::update_stats_internal(&mut stats, &result.result);
        if let Some(coverage) = &result.coverage {
            self.coverage_tracker.merge(&CoverageTracker {
                data: Arc::new(Mutex::new(coverage.clone())),
            });
        }
    }

    fn update_stats(stats: &Arc<Mutex<FuzzerStats>>, result: &FuzzResult) {
        let mut stats = stats.lock().unwrap();
        Self::update_stats_internal(&mut stats, &result.result);
    }

    fn update_stats_internal(stats: &mut FuzzerStats, result: &ExecutionResult) {
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
}
