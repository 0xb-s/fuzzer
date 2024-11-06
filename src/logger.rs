use crate::coverage::CoverageData;
use crate::errors::FuzzerError;
use crate::fuzz_engine::FuzzerStats;

use chrono::Local;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub struct Logger {
    log_file: Option<String>,
    html_report_file: Option<String>,
    json_report_file: Option<String>,
}

impl Logger {
    pub fn new(
        log_file: Option<String>,
        html_report_file: Option<String>,
        json_report_file: Option<String>,
    ) -> Self {
        Logger {
            log_file,
            html_report_file,
            json_report_file,
        }
    }

    pub fn log_stats(&self, stats: &FuzzerStats) -> Result<(), FuzzerError> {
        if let Some(ref file) = self.log_file {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(file)
                .unwrap();
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            writeln!(file, "[{}] Stats: {:?}", timestamp, stats).unwrap();
        }
        Ok(())
    }

    pub fn generate_html_report(
        &self,
        stats: &FuzzerStats,
        coverage_data: &CoverageData,
    ) -> Result<(), FuzzerError> {
        if let Some(ref file) = self.html_report_file {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file)
                .unwrap();
            let html_content = format!(
                "<html><head><title>Fuzzing Report</title></head><body>\
                <h1>Fuzzing Statistics</h1>\
                <p>Total Runs: {}</p>\
                <p>Successful Runs: {}</p>\
                <p>Errors: {}</p>\
                <p>Timeouts: {}</p>\
                <p>Unique Crashes: {}</p>\
                <p>Total Crashes: {}</p>\
                <p>Inputs Tested: {}</p>\
                <h2>Coverage Data</h2>\
                <p>Blocks Covered: {}</p>\
                </body></html>",
                stats.total_runs,
                stats.successful_runs,
                stats.errors,
                stats.timeouts,
                stats.unique_crashes.len(),
                stats.total_crashes,
                stats.inputs_tested,
                coverage_data.covered_blocks.len()
            );
            file.write_all(html_content.as_bytes()).unwrap();
        }
        Ok(())
    }

    pub fn generate_json_report(
        &self,
        stats: &FuzzerStats,
        coverage_data: &CoverageData,
    ) -> Result<(), FuzzerError> {
        if let Some(ref file) = self.json_report_file {
            let report = json!({
                "total_runs": stats.total_runs,
                "successful_runs": stats.successful_runs,
                "errors": stats.errors,
                "timeouts": stats.timeouts,
                "unique_crashes": stats.unique_crashes.len(),
                "total_crashes": stats.total_crashes,
                "inputs_tested": stats.inputs_tested,
                "elapsed_time": stats.total_time.as_secs(),
                "coverage": {
                    "blocks_covered": coverage_data.covered_blocks.len(),
                    "block_hit_counts": coverage_data.block_hit_counts,
                },
            });

            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file)
                .unwrap();
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &report).unwrap();
        }
        Ok(())
    }
}
