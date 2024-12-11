use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::input::{Input, Result};

#[derive(Debug, Clone)]
pub struct InputFiles {
    base_path: PathBuf,
}

impl InputFiles {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path)?;

        Ok(Self { base_path })
    }

    pub fn filepath(&self, day: usize) -> PathBuf {
        self.base_path.join(format!("day{day}"))
    }
}

impl Input for InputFiles {
    fn has_input(&self, day: usize) -> bool {
        self.filepath(day).exists()
    }

    fn get_input(&mut self, day: usize) -> Result<String> {
        let mut file = File::open(self.filepath(day))?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        Ok(input)
    }
}
