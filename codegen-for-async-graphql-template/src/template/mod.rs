mod mod_file;
mod object_type;
mod save;
mod scalar;
mod utils;

pub use mod_file::generate_file as generate_mod_file;
pub use object_type::{generate_object_type_file, generate_object_types_token_stream};
pub use save::Save;
pub use scalar::generate_scalar_type_file;

#[derive(Debug, Clone)]
pub struct BuildingScalar {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct BuildingObjectType {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct BuildingStatus {
    pub scalars: Vec<BuildingScalar>,
    pub object_types: Vec<BuildingObjectType>,
}

impl BuildingStatus {
    pub fn names(&self) -> Vec<String> {
        let mut scalar_names = self.scalar_path_names();
        let object_type_names: Vec<String> =
            self.object_types.iter().map(|f| f.name.clone()).collect();
        scalar_names.extend(object_type_names);
        scalar_names
    }

    pub fn scalar_path_names(&self) -> Vec<String> {
        self.scalars.iter().map(|f| f.path.clone()).collect()
    }

    pub fn scalar_names(&self) -> Vec<String> {
        self.scalars.iter().map(|f| f.name.clone()).collect()
    }
}
