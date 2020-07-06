use std::fs;
use std::io::Write;
use std::path::Path;

use super::Context;

use std::process::{Command, Stdio};

fn path_format(name: &str, path: &str) -> String {
    format!("{}/{}.rs", path, name)
}

fn create_or_get_path(base_path: &str, super_module_name: Option<String>) -> String {
    match super_module_name {
        Some(f) => {
            let d = format!("{}/{}", base_path, f);
            if !Path::new(&d).exists() {
                fs::create_dir(&d).expect("Error");
            };
            d
        }
        None => base_path.to_string(),
    }
}

fn prepare_path(base_path: &str, super_module_name: Option<String>, file_name: &str) -> String {
    if !Path::new(base_path).exists() {
        panic!("Does not exist: {}", base_path);
    }
    let dir = create_or_get_path(base_path, super_module_name);
    path_format(file_name, dir.as_str())
}

pub fn lint(path: &str) {
    let rustfmt = toolchain_find::find_installed_component("rustfmt").unwrap();
    Command::new(&rustfmt)
        .arg("--edition=2018")
        .arg(path)
        .stderr(Stdio::null())
        .output()
        .expect("rustfmt error");
}

pub trait Save {
    fn file_name(&self) -> String;

    fn str_src(&self) -> String;

    fn super_module_name(&self) -> Option<String>;

    fn save(&self, context: &Context) {
        let path = prepare_path(
            &context.config.output_bnase_path,
            self.super_module_name(),
            &self.file_name(),
        );
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(self.str_src().as_bytes()).unwrap();
        lint(&path);
    }
}
