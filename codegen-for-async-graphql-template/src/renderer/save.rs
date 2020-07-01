use std::fs;
use std::io::Write;

use super::Context;

fn path_format(name: &str, path: &str) -> String {
    format!("{}/{}.rs", path, name)
}

use std::process::{Command, Stdio};

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
    fn relative_path(&self) -> String;

    fn str_src(&self) -> String;

    fn save(&self, context: &Context) {
        let path = path_format(&self.relative_path(), &context.config.output_bnase_path);
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(self.str_src().as_bytes()).unwrap();
        lint(&path);
    }
}
