use crate::config::FuzzerConfig;
use crate::errors::FuzzerError;
use crate::utils::InputFormat;
use rand::{prelude::StdRng, Rng, SeedableRng};
use serde_json::json;
#[derive(Clone, Debug)]
pub struct InputGenerator {
    config: FuzzerConfig,
    rng: StdRng,
}

impl InputGenerator {
    pub fn new(config: FuzzerConfig) -> Self {
        let seed = config.seed.unwrap_or_else(|| rand::thread_rng().gen());
        InputGenerator {
            config,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_input(&mut self) -> Result<Vec<u8>, FuzzerError> {
        match self.config.input_format {
            InputFormat::Binary => self.generate_binary(),
            InputFormat::Text => self.generate_text(),
            InputFormat::JSON => self.generate_json(),
            InputFormat::XML => self.generate_xml(),
            InputFormat::Custom(ref format) => self.generate_custom(format.clone().as_str()),
            //todo
            _ => self.generate_binary(),
        }
    }

    fn generate_binary(&mut self) -> Result<Vec<u8>, FuzzerError> {
        let size = self.rng.gen_range(1..1024);
        Ok((0..size).map(|_| self.rng.gen::<u8>()).collect())
    }

    fn generate_text(&mut self) -> Result<Vec<u8>, FuzzerError> {
        let size = self.rng.gen_range(1..1024);
        let text: String = (0..size)
            .map(|_| self.rng.gen_range(32u8..127u8) as char)
            .collect();
        Ok(text.into_bytes())
    }

    fn generate_json(&mut self) -> Result<Vec<u8>, FuzzerError> {
        let obj = json!({
            "key": self.rng.gen::<u64>(),
            "value": self.rng.gen::<f64>(),
            "array": [self.rng.gen::<bool>(), self.rng.gen::<bool>()],
            "nested": {
                "a": self.rng.gen::<u8>(),
                "b": format!("str{}", self.rng.gen::<u8>()),
            }
        });
        serde_json::to_vec(&obj).map_err(|e| FuzzerError::InputGenerationError(e.to_string()))
    }

    fn generate_xml(&mut self) -> Result<Vec<u8>, FuzzerError> {
        let xml = format!(
            "<root><value>{}</value><flag>{}</flag></root>",
            self.rng.gen::<u64>(),
            self.rng.gen::<bool>()
        );
        Ok(xml.into_bytes())
    }

    fn generate_custom(&mut self, format: &str) -> Result<Vec<u8>, FuzzerError> {
        // Custom format handling logic
        let data = format!("Custom format: {}", format);
        Ok(data.into_bytes())
    }
}
