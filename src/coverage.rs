use crate::errors::FuzzerError;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoverageData {
    pub covered_blocks: HashSet<usize>,
    pub block_hit_counts: HashMap<usize, usize>,
}

impl CoverageData {
    pub fn new() -> Self {
        CoverageData {
            covered_blocks: HashSet::new(),
            block_hit_counts: HashMap::new(),
        }
    }

    pub fn record_block(&mut self, block_id: usize) {
        self.covered_blocks.insert(block_id);
        *self.block_hit_counts.entry(block_id).or_insert(0) += 1;
    }

    pub fn merge(&mut self, other: &CoverageData) {
        for &block_id in &other.covered_blocks {
            self.covered_blocks.insert(block_id);
            *self.block_hit_counts.entry(block_id).or_insert(0) +=
                other.block_hit_counts.get(&block_id).copied().unwrap_or(0);
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), FuzzerError> {
        let file = File::create(filename).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self).unwrap();
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, FuzzerError> {
        let file = File::open(filename).unwrap();
        let coverage_data = serde_json::from_reader(file).unwrap();
        Ok(coverage_data)
    }
}

#[derive(Clone, Debug)]
pub struct CoverageTracker {
    pub data: Arc<Mutex<CoverageData>>,
}

impl CoverageTracker {
    pub fn new() -> Self {
        CoverageTracker {
            data: Arc::new(Mutex::new(CoverageData::new())),
        }
    }

    pub fn record(&self, block_id: usize) {
        let mut data = self.data.lock().unwrap();
        data.record_block(block_id);
    }

    pub fn get_coverage(&self) -> CoverageData {
        let data = self.data.lock().unwrap();
        data.clone()
    }

    pub fn merge(&self, other: &CoverageTracker) {
        let mut data = self.data.lock().unwrap();
        let other_data = other.data.lock().unwrap();
        data.merge(&other_data);
    }

    pub fn save_coverage(&self, filename: &str) -> Result<(), FuzzerError> {
        let data = self.data.lock().unwrap();
        data.save_to_file(filename)
    }

    pub fn load_coverage(&self, filename: &str) -> Result<(), FuzzerError> {
        let coverage_data = CoverageData::load_from_file(filename)?;
        let mut data = self.data.lock().unwrap();
        data.merge(&coverage_data);
        Ok(())
    }
}
