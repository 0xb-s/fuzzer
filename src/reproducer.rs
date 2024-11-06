use crate::errors::FuzzerError;
use crate::target::{Executable, TargetFunction};

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub struct CrashReproducer {
    target: TargetFunction,
}

impl CrashReproducer {
    pub fn new(target: TargetFunction) -> Self {
        CrashReproducer { target }
    }

    pub async fn reproduce_from_file(&self, file_path: &str) -> Result<(), FuzzerError> {
        let mut file = File::open(file_path).unwrap();
        let mut crash_input = Vec::new();
        file.read_to_end(&mut crash_input).unwrap();
        self.reproduce(&crash_input).await
    }

    pub async fn reproduce(&self, crash_input: &[u8]) -> Result<(), FuzzerError> {
        match self.target.execute(crash_input).await {
            Ok(_) => Err(FuzzerError::ReproductionFailed(
                "No crash occurred".to_string(),
            )),
            Err(e) => {
                println!("Crash reproduced: {}", e);
                Ok(())
            }
        }
    }

    pub async fn reproduce_and_log(
        &self,
        crash_input: &[u8],
        log_file: &str,
    ) -> Result<(), FuzzerError> {
        match self.target.execute(crash_input).await {
            Ok(_) => Err(FuzzerError::ReproductionFailed(
                "No crash occurred".to_string(),
            )),
            Err(e) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(log_file)
                    .unwrap();
                writeln!(file, "Crash reproduced: {}", e).unwrap();
                Ok(())
            }
        }
    }
}
