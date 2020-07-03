mod config;
mod context;
mod generator;
pub mod utils;

pub use generator::generate_from_path;

pub use config::Config;
pub use context::Context;

use crate::renderer::render_to_files;
