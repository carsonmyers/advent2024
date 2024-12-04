mod error;
mod input;

pub use error::Error;
pub use input::Input;

pub type Result<T> = std::result::Result<T, Error>;
