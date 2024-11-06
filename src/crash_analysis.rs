use crate::errors::FuzzerError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrashAnalysis {
    pub crashes: HashMap<String, CrashInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrashInfo {
    pub crash_hash: String,
    pub input: Vec<u8>,
    pub stack_trace: Option<String>,
    pub severity: CrashSeverity,
    pub exploitability: Exploitability,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CrashSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Exploitability {
    None,
    Potential,
    Probable,
    Proven,
}

impl CrashAnalysis {
    pub fn new() -> Self {
        CrashAnalysis {
            crashes: HashMap::new(),
        }
    }

    pub fn analyze_crash(
        &mut self,
        crash_input: &[u8],
        crash_info: &str,
    ) -> Result<(), FuzzerError> {
        let crash_hash = self.calculate_crash_hash(crash_info);
        if !self.crashes.contains_key(&crash_hash) {
            let severity = self.determine_severity(crash_info);
            let exploitability = self.determine_exploitability(crash_info);
            let crash_info = CrashInfo {
                crash_hash: crash_hash.clone(),
                input: crash_input.to_vec(),
                stack_trace: Some(crash_info.to_string()),
                severity,
                exploitability,
            };
            self.crashes.insert(crash_hash, crash_info);
        }
        Ok(())
    }

    fn calculate_crash_hash(&self, crash_info: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(crash_info.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn determine_severity(&self, crash_info: &str) -> CrashSeverity {
        if crash_info.contains("buffer overflow") {
            CrashSeverity::Critical
        } else if crash_info.contains("null pointer") {
            CrashSeverity::High
        } else {
            CrashSeverity::Medium
        }
    }

    fn determine_exploitability(&self, crash_info: &str) -> Exploitability {
        if crash_info.contains("control over EIP") {
            Exploitability::Proven
        } else if crash_info.contains("heap corruption") {
            Exploitability::Probable
        } else {
            Exploitability::Potential
        }
    }

    pub fn save_crash_analysis(&self, filename: &str) -> Result<(), FuzzerError> {
        let file = File::create(filename).unwrap();
        serde_json::to_writer_pretty(file, &self).unwrap();
        Ok(())
    }

    pub fn load_crash_analysis(filename: &str) -> Result<Self, FuzzerError> {
        let file = File::open(filename).unwrap();
        let analysis = serde_json::from_reader(file).unwrap();
        Ok(analysis)
    }
}
