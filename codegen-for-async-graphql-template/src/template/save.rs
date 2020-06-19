use std::fs;
use std::io::Write;

fn path_format(name: &String) -> String {
    format!("./src/models/{}.rs", name)
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
    fn save(name: &String, src: &String) {
        let path = path_format(name);
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(src.as_bytes()).unwrap();
        lint(&path);
    }
}
