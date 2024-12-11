use crate::input::{Input, Result};

#[derive(Debug)]
pub struct SimpleInput {
    input: String,
}

impl SimpleInput {
    #[cfg(test)]
    pub fn new<S: Into<String>>(input: S) -> SimpleInput {
        Self {
            input: input.into(),
        }
    }
}

impl Input for SimpleInput {
    fn has_input(&self, _: usize) -> bool {
        true
    }

    fn get_input(&mut self, _: usize) -> Result<String> {
        Ok(self.input.clone())
    }
}
