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

#[derive(Debug, Clone)]
pub struct BuildingStatus {
    pub scalars: Vec<BuildingScalar>,
    pub object_types: Vec<BuildingObjectType>,
}

impl BuildingStatus {
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        let mut scalar_names = self.scalar_path_names();
        let object_type_names: Vec<String> =
            self.object_types.iter().map(|f| f.name.clone()).collect();
        scalar_names.extend(object_type_names);
        scalar_names
    }

    #[must_use]
    pub fn scalar_path_names(&self) -> Vec<String> {
        self.scalars.iter().map(|f| f.path.clone()).collect()
    }

    #[must_use]
    pub fn scalar_names(&self) -> Vec<String> {
        self.scalars.iter().map(|f| f.name.clone()).collect()
    }
}
