mod mod_file;
mod object_type;
mod save;
mod scalar;
mod utils;

pub use mod_file::generate_file as generate_mod_file;
pub use object_type::{generate_object_type_file, generate_object_types_token_stream};
pub use save::Save;
pub use scalar::generate_scalar_type_file;

#[derive(Debug)]
pub struct BuildingScalar {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct BuildingObjectType {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct BuildingStatus {
    pub scalars: Vec<BuildingScalar>,
    pub object_types: Vec<BuildingObjectType>,
}
