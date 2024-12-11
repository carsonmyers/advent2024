mod download;
mod error;
pub mod helpers;
mod input_files;
mod simple;

use std::fmt::Debug;

pub use download::Download;
pub use error::Error;
pub use input_files::InputFiles;

#[cfg(test)]
pub use simple::SimpleInput;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Input: Debug + Send + Sync {
    fn has_input(&self, day: usize) -> bool;
    fn get_input(&mut self, day: usize) -> Result<String>;
}
